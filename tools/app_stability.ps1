# P-24: the application's own stability and responsiveness, measured from outside the window.
#
# `cargo test` cannot start a window, drag it by its title bar or watch it paint, so this script
# drives the real release build the way a person does: it starts the program, photographs the
# screen while it comes up, drags the title bar and the corner with real mouse input (SendInput),
# asks the window every 50 ms whether it is still answering, and closes it the way the rebuild
# watcher does. Run from the repository root, with nothing else heavy running:
#
#     cargo build -p anime_compositor_app --release
#     powershell -ExecutionPolicy Bypass -File tools/app_stability.ps1
#
# The mouse is taken over for the whole run (about 45 minutes with the 20-minute soak); leave the
# machine alone. Every press is checked first: the script only presses where the window under the
# pointer is the application's own and Windows says the point is its title bar or its corner, and
# it waits for the person to have left the mouse and keyboard alone for two seconds, so a window
# that failed to come to the front is recorded as skipped rather than clicked through.
#
# It writes verification/P-24_app_stability_table.md and the photographs P-24_*.png beside it
# (-Tag B-174 writes B-174_* instead). -Part runs some parts (startup, remember, window, cycles,
# soak; several with commas) for a quicker look; the table then says which parts ran. The remembered window (B-174's
# window.txt) is put back as it was found too; every start-up launch begins without one, as a
# first launch does. The projects it opens are copies made in a folder of its own under %TEMP%, deleted at
# the end with every file the runs created, and the recent-projects list is put back exactly as it
# was found.
#
# Nothing here changes the application: the photographs are taken with the GPU on, exactly as the
# owner runs it. The window and soak parts, and the first-picture launches, start the web view with
# a debugging port so the script can read the frame number the page shows and press Play; the
# table says so beside every figure that used it. The other launches run without it.

param(
  [ValidateSet('all','startup','remember','window','cycles','soak')][string[]]$Part = 'all',
  # The name the table and the photographs are written under: a later unit's re-run keeps P-24's.
  [string]$Tag = 'P-24',
  [int]$Launches = 10,
  [int]$Cycles = 50,
  [int]$SoakMinutes = 20
)

$ErrorActionPreference = 'Stop'
$root = Split-Path -Parent $PSScriptRoot
$exe = Join-Path $root 'target\release\anime_compositor_app.exe'
if (-not (Test-Path $exe)) { throw 'build it first: cargo build -p anime_compositor_app --release' }
$out = Join-Path $root 'verification'
function Runs($p) { $Part -contains 'all' -or $Part -contains $p }

Add-Type -AssemblyName System.Drawing
Add-Type -ReferencedAssemblies System.Drawing -TypeDefinition @'
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Drawing.Imaging;
using System.Net.WebSockets;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.RegularExpressions;
using System.Threading;

public static class H {
  [DllImport("user32.dll")] public static extern bool SetProcessDPIAware();
  [DllImport("winmm.dll")] public static extern uint timeBeginPeriod(uint ms);
  [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool GetClientRect(IntPtr h, out RECT r);
  [DllImport("user32.dll")] public static extern bool ClientToScreen(IntPtr h, ref POINT p);
  [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsZoomed(IntPtr h);
  [DllImport("user32.dll")] public static extern bool IsHungAppWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern IntPtr GetForegroundWindow();
  [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
  [DllImport("user32.dll")] public static extern bool BringWindowToTop(IntPtr h);
  [DllImport("user32.dll")] public static extern bool SetWindowPos(IntPtr h, IntPtr after, int x, int y, int w, int hh, uint flags);
  [DllImport("user32.dll")] public static extern uint GetWindowThreadProcessId(IntPtr h, out uint pid);
  [DllImport("user32.dll")] public static extern bool AttachThreadInput(uint a, uint b, bool attach);
  [DllImport("kernel32.dll")] public static extern uint GetCurrentThreadId();
  [DllImport("user32.dll")] public static extern IntPtr WindowFromPoint(POINT p);
  [DllImport("user32.dll")] public static extern IntPtr GetAncestor(IntPtr h, uint flags);
  [DllImport("user32.dll")] public static extern bool PostMessage(IntPtr h, uint msg, IntPtr w, IntPtr l);
  [DllImport("user32.dll")] public static extern IntPtr SendMessageTimeout(IntPtr h, uint msg, IntPtr w, IntPtr l, uint flags, uint timeout, out IntPtr result);
  [DllImport("user32.dll")] public static extern int GetSystemMetrics(int i);
  [DllImport("user32.dll")] public static extern uint SendInput(uint n, INPUT[] inputs, int size);
  [DllImport("user32.dll")] public static extern bool GetLastInputInfo(ref LASTINPUTINFO i);
  [DllImport("user32.dll")] public static extern bool EnumWindows(EnumProc f, IntPtr l);
  [DllImport("user32.dll")] public static extern int GetWindowText(IntPtr h, StringBuilder s, int n);
  [DllImport("kernel32.dll")] public static extern IntPtr CreateToolhelp32Snapshot(uint flags, uint pid);
  [DllImport("kernel32.dll")] public static extern bool Process32FirstW(IntPtr s, ref PROCESSENTRY32 e);
  [DllImport("kernel32.dll")] public static extern bool Process32NextW(IntPtr s, ref PROCESSENTRY32 e);
  [DllImport("kernel32.dll")] public static extern bool CloseHandle(IntPtr h);
  public delegate bool EnumProc(IntPtr h, IntPtr l);

  [StructLayout(LayoutKind.Sequential)] public struct RECT { public int L, T, R, B; }
  [StructLayout(LayoutKind.Sequential)] public struct POINT { public int X, Y; }
  [StructLayout(LayoutKind.Sequential)] public struct LASTINPUTINFO { public uint cbSize, dwTime; }
  [StructLayout(LayoutKind.Sequential)] public struct MOUSEINPUT { public int dx, dy; public uint data, flags, time; public IntPtr extra; }
  // 40 bytes on 64-bit Windows, as SendInput insists: the mouse member is the union's largest.
  [StructLayout(LayoutKind.Sequential)] public struct INPUT { public uint type; public MOUSEINPUT mi; }
  [DllImport("user32.dll")] public static extern bool GetCursorPos(out POINT p);
  [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int cmd);
  [DllImport("user32.dll")] public static extern IntPtr MonitorFromPoint(POINT p, uint flags);
  [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Unicode)] public struct PROCESSENTRY32 {
    public uint dwSize, cntUsage, th32ProcessID; public IntPtr th32DefaultHeapID; public uint th32ModuleID, cntThreads, th32ParentProcessID;
    public int pcPriClassBase; public uint dwFlags; [MarshalAs(UnmanagedType.ByValTStr, SizeConst = 260)] public string szExeFile; }

  public static readonly Stopwatch Clock = Stopwatch.StartNew();
  public static double Now() { return Clock.Elapsed.TotalMilliseconds; }

  // ---- processes -------------------------------------------------------------------------

  // The process and every process below it: the application, its WebView2 browser process and
  // that browser's renderer, GPU and utility processes. Other programs' web views are not in it.
  public static List<int> Tree(int pid) {
    var parent = new Dictionary<int, int>();
    IntPtr s = CreateToolhelp32Snapshot(2, 0);
    var e = new PROCESSENTRY32(); e.dwSize = (uint)Marshal.SizeOf(typeof(PROCESSENTRY32));
    if (Process32FirstW(s, ref e)) do { parent[(int)e.th32ProcessID] = (int)e.th32ParentProcessID; } while (Process32NextW(s, ref e));
    CloseHandle(s);
    var all = new List<int> { pid };
    for (int i = 0; i < all.Count; i++) foreach (var kv in parent) if (kv.Value == all[i] && kv.Key != all[i] && !all.Contains(kv.Key)) all.Add(kv.Key);
    return all;
  }
  // Still running, and still one of ours rather than a number Windows has handed to something new.
  public static bool Alive(int pid) {
    try { var p = Process.GetProcessById(pid); if (p.HasExited) return false; string n = p.ProcessName.ToLower(); return n == "anime_compositor_app" || n == "msedgewebview2"; }
    catch { return false; }
  }
  // Working set and private bytes of a set of processes, summed, in bytes.
  public static long[] Memory(List<int> pids) {
    long ws = 0, priv = 0;
    foreach (int p in pids) { try { var pr = Process.GetProcessById(p); ws += pr.WorkingSet64; priv += pr.PrivateMemorySize64; } catch { } }
    return new long[] { ws, priv };
  }
  public static double Cpu(List<int> pids) {
    double t = 0;
    foreach (int p in pids) { try { t += Process.GetProcessById(p).TotalProcessorTime.TotalMilliseconds; } catch { } }
    return t;
  }
  public static void KillTree(List<int> pids) { foreach (int p in pids) { try { if (Alive(p)) Process.GetProcessById(p).Kill(); } catch { } } }

  // The application's top-level window, or zero. Its title is the one tauri.conf.json gives it.
  public static IntPtr Window(int pid) {
    IntPtr found = IntPtr.Zero;
    EnumWindows((h, l) => {
      uint p; GetWindowThreadProcessId(h, out p);
      if (p != pid) return true;
      var s = new StringBuilder(64); GetWindowText(h, s, 64);
      if (s.ToString() == "Anime Compositor") { found = h; return false; }
      return true; }, IntPtr.Zero);
    return found;
  }

  // ---- the window ------------------------------------------------------------------------

  // To the front for certain. Windows refuses SetForegroundWindow to a process the person has not
  // touched; borrowing the current foreground thread's input state is the documented way round it
  // and presses nothing.
  public static bool Front(IntPtr h) {
    for (int i = 0; i < 20; i++) {
      IntPtr fg = GetForegroundWindow();
      if (fg == h) return true;
      uint dummy; uint fgThread = GetWindowThreadProcessId(fg, out dummy);
      uint me = GetCurrentThreadId();
      AttachThreadInput(me, fgThread, true);
      BringWindowToTop(h); SetForegroundWindow(h);
      AttachThreadInput(me, fgThread, false);
      Thread.Sleep(50);
    }
    return GetForegroundWindow() == h;
  }
  // Which program a window belongs to, and its title: who took the front.
  public static string Who(IntPtr h) {
    uint p; GetWindowThreadProcessId(h, out p);
    var s = new StringBuilder(80); GetWindowText(h, s, 80);
    string name = "?"; try { name = Process.GetProcessById((int)p).ProcessName; } catch { }
    return name + " '" + s + "'";
  }
  public static RECT Rect(IntPtr h) { RECT r; GetWindowRect(h, out r); return r; }
  public static RECT Client(IntPtr h) {
    RECT c; GetClientRect(h, out c); var p = new POINT(); ClientToScreen(h, ref p);
    return new RECT { L = p.X, T = p.Y, R = p.X + c.R, B = p.Y + c.B };
  }
  // A maximized window is restored first (B-174 opens maximized): SetWindowPos alone would move a
  // window that still believes it is maximized.
  public static void Place(IntPtr h, int x, int y, int w, int hh) {
    if (IsZoomed(h) || IsIconic(h)) { ShowWindow(h, 9); Thread.Sleep(500); }
    SetWindowPos(h, IntPtr.Zero, x, y, w, hh, 0x0004 | 0x0010);
  }
  // True if the middle of the window's top edge is on some display: a person can reach its title bar.
  public static bool OnScreen(IntPtr h) { RECT r = Rect(h); return MonitorFromPoint(new POINT { X = (r.L + r.R) / 2, Y = r.T + 20 }, 0) != IntPtr.Zero; }
  // True only if the window under (x, y) is this one: nothing is pressed anywhere else.
  public static bool Ours(IntPtr h, int x, int y) { return GetAncestor(WindowFromPoint(new POINT { X = x, Y = y }), 2) == h; }
  // What Windows says the point is: 2 the title bar, 17 the bottom-right corner.
  public static int HitTest(IntPtr h, int x, int y) {
    IntPtr r; SendMessageTimeout(h, 0x84, IntPtr.Zero, (IntPtr)(((y & 0xFFFF) << 16) | (x & 0xFFFF)), 0, 2000, out r);
    return (int)r;
  }
  // How long the window takes to answer a message that does nothing, in ms; negative if it gave up.
  public static double Ping(IntPtr h, uint timeout) {
    double t0 = Now(); IntPtr r;
    IntPtr ok = SendMessageTimeout(h, 0, IntPtr.Zero, IntPtr.Zero, 0, timeout, out r);
    double t = Now() - t0;
    return ok == IntPtr.Zero ? -t : t;
  }
  // Milliseconds since anyone (the person, or this script) last touched the mouse or keyboard.
  public static uint IdleMs() { var i = new LASTINPUTINFO { cbSize = 8 }; GetLastInputInfo(ref i); return (uint)Environment.TickCount - i.dwTime; }

  // ---- the mouse -------------------------------------------------------------------------

  static bool Send(uint flags, int x, int y) {
    int vx = GetSystemMetrics(76), vy = GetSystemMetrics(77), vw = GetSystemMetrics(78), vh = GetSystemMetrics(79);
    var i = new INPUT { type = 0 };
    i.mi.dx = (int)Math.Round((x - vx + 0.5) * 65536.0 / vw);
    i.mi.dy = (int)Math.Round((y - vy + 0.5) * 65536.0 / vh);
    i.mi.flags = flags | 0x8000 | 0x4000 | 0x0001;   // absolute, whole virtual desktop, move
    return SendInput(1, new[] { i }, Marshal.SizeOf(typeof(INPUT))) == 1;
  }
  // True if the pointer is within a pixel of (x, y).
  static bool At(int x, int y) { POINT p; GetCursorPos(out p); return Math.Abs(p.X - x) <= 1 && Math.Abs(p.Y - y) <= 1; }

  // Presses at (x0, y0), moves through the offsets one every stepMs, lets go at the last one.
  // Refuses, pressing nothing, unless the point is this window's and Windows calls it `hit`.
  public static string Drag(IntPtr h, int x0, int y0, int[] dx, int[] dy, int stepMs, int hit) {
    if (!Front(h)) return "the window would not come to the front";
    if (!Ours(h, x0, y0)) return "another window was under the pointer";
    int got = HitTest(h, x0, y0);
    if (got != hit) return "the point was not the expected part of the window (" + got + ")";
    if (!Send(0, x0, y0)) return "Windows refused the mouse input";
    Thread.Sleep(150);
    if (!At(x0, y0)) return "the pointer did not arrive where it was sent";
    if (!Ours(h, x0, y0) || GetForegroundWindow() != h) return "the window lost the front before the press";
    Send(0x0002, x0, y0);
    try {
      Thread.Sleep(150);
      double t0 = Now();
      for (int i = 0; i < dx.Length; i++) {
        Send(0, x0 + dx[i], y0 + dy[i]);
        double wait = t0 + (i + 1) * stepMs - Now(); if (wait > 0) Thread.Sleep((int)wait);
      }
    } finally {
      Send(0x0004, x0 + dx[dx.Length - 1], y0 + dy[dy.Length - 1]);
    }
    return null;
  }
  // A point Windows calls the title bar, a little in from the left; or the bottom-right corner.
  public static POINT Caption(IntPtr h) {
    RECT r = Rect(h), c = Client(h);
    for (int x = r.L + 300; x < r.R - 300; x += 40) { int y = (r.T + c.T) / 2 + 4; if (HitTest(h, x, y) == 2 && Ours(h, x, y)) return new POINT { X = x, Y = y }; }
    return new POINT { X = -1, Y = -1 };
  }
  public static POINT Corner(IntPtr h) {
    RECT r = Rect(h);
    for (int d = 1; d < 30; d++) { int x = r.R - d, y = r.B - d; if (HitTest(h, x, y) == 17 && Ours(h, x, y)) return new POINT { X = x, Y = y }; }
    return new POINT { X = -1, Y = -1 };
  }

  // ---- photographs -----------------------------------------------------------------------

  // A copy of the screen where the rectangle is: what a person sees, GPU drawing included.
  public static Bitmap Grab(RECT r) {
    var b = new Bitmap(Math.Max(1, r.R - r.L), Math.Max(1, r.B - r.T), PixelFormat.Format32bppArgb);
    using (var g = Graphics.FromImage(b)) g.CopyFromScreen(r.L, r.T, 0, 0, b.Size, CopyPixelOperation.SourceCopy);
    return b;
  }
  // [hash, distinct colours in a 64x64 sample (counting stops at 9), white share of that sample in thousandths]
  public static long[] Look(Bitmap b, Rectangle part) {
    part.Intersect(new Rectangle(0, 0, b.Width, b.Height));
    if (part.Width <= 0 || part.Height <= 0) return new long[] { 0, 0, 0 };
    var d = b.LockBits(part, ImageLockMode.ReadOnly, PixelFormat.Format32bppArgb);
    int n = d.Stride * part.Height; var px = new byte[n]; Marshal.Copy(d.Scan0, px, 0, n); b.UnlockBits(d);
    ulong hash = 1469598103934665603UL;
    for (int y = 0; y < part.Height; y++) for (int x = 0, o = y * d.Stride; x < part.Width; x++, o += 4)
      hash = (hash ^ (ulong)(px[o] | px[o + 1] << 8 | px[o + 2] << 16)) * 1099511628211UL;
    var colours = new HashSet<int>(); int white = 0;
    for (int sy = 0; sy < 64; sy++) for (int sx = 0; sx < 64; sx++) {
      int x = (int)((sx + 0.5) * part.Width / 64), y = (int)((sy + 0.5) * part.Height / 64);
      int o = y * d.Stride + x * 4;
      if (colours.Count < 9) colours.Add(px[o] | px[o + 1] << 8 | px[o + 2] << 16);
      if (px[o] > 245 && px[o + 1] > 245 && px[o + 2] > 245) white++;
    }
    return new long[] { (long)hash, colours.Count, white * 1000L / 4096 };
  }
  // Blank: one or two colours, or nine tenths white. The application's own interface is dark and
  // busy, so neither happens once it has drawn.
  public static bool Blank(long[] look) { return look[1] < 3 || look[2] > 900; }
  public static Rectangle All(Bitmap b) { return new Rectangle(0, 0, b.Width, b.Height); }
  // Where two photographs of the same size differ: "none", or the count of differing pixels and
  // the rectangle around them, in pixels of the photograph.
  public static string Diff(Bitmap a, Bitmap b) {
    if (a.Width != b.Width || a.Height != b.Height) return "different sizes";
    var r = All(a);
    var da = a.LockBits(r, ImageLockMode.ReadOnly, PixelFormat.Format32bppArgb); var db = b.LockBits(r, ImageLockMode.ReadOnly, PixelFormat.Format32bppArgb);
    int n = da.Stride * r.Height; var pa = new byte[n]; var pb = new byte[n];
    Marshal.Copy(da.Scan0, pa, 0, n); Marshal.Copy(db.Scan0, pb, 0, n); a.UnlockBits(da); b.UnlockBits(db);
    int count = 0, x0 = int.MaxValue, y0 = int.MaxValue, x1 = -1, y1 = -1;
    for (int y = 0; y < r.Height; y++) for (int x = 0, o = y * da.Stride; x < r.Width; x++, o += 4)
      if (pa[o] != pb[o] || pa[o + 1] != pb[o + 1] || pa[o + 2] != pb[o + 2]) { count++; x0 = Math.Min(x0, x); y0 = Math.Min(y0, y); x1 = Math.Max(x1, x); y1 = Math.Max(y1, y); }
    return count == 0 ? "none" : count + " pixels within " + (x1 - x0 + 1) + "x" + (y1 - y0 + 1) + " at " + x0 + "," + y0;
  }

  // Waits for the window's picture to stop changing: returns the ms from `since` to the last
  // change before `still` ms without one, or -1 if `limit` passed first. Keeps the last photograph.
  public static Bitmap Settled;
  public static double Settle(IntPtr h, double since, double still, double limit) {
    long last = 0; double changed = -1;
    if (Settled != null) { Settled.Dispose(); Settled = null; }
    while (Now() - since < limit) {
      double t = Now();
      var b = Grab(Client(h));
      var l = Look(b, All(b));
      if (l[0] != last || Settled == null) { last = l[0]; changed = t - since; if (Settled != null) Settled.Dispose(); Settled = b; }
      else { b.Dispose(); if (!Blank(l) && t - since - changed >= still) return changed; }
    }
    return -1;
  }

  // ---- the page, through the web view's debugging port -----------------------------------

  public class Page : IDisposable {
    ClientWebSocket ws; int id;
    // Waits up to `limit` ms for the port to answer; null if it never did.
    public static Page Open(int port, double limit) {
      double t0 = Now();
      while (Now() - t0 < limit) {
        try {
          string list = new System.Net.WebClient().DownloadString("http://127.0.0.1:" + port + "/json");
          var m = Regex.Match(list, "\"type\":\\s*\"page\"[\\s\\S]*?\"webSocketDebuggerUrl\":\\s*\"([^\"]+)\"");
          if (!m.Success) m = Regex.Match(list, "\"webSocketDebuggerUrl\":\\s*\"([^\"]+)\"");
          if (m.Success) {
            var p = new Page(); p.ws = new ClientWebSocket();
            p.ws.ConnectAsync(new Uri(m.Groups[1].Value), CancellationToken.None).Wait();
            return p;
          }
        } catch { }
        Thread.Sleep(10);
      }
      return null;
    }
    // The value of a JavaScript expression, as the text of its JSON, or null.
    public string Eval(string js) {
      lock (this) {
        int me = ++id;
        string msg = "{\"id\":" + me + ",\"method\":\"Runtime.evaluate\",\"params\":{\"expression\":" + Quote(js) + ",\"returnByValue\":true}}";
        var bytes = Encoding.UTF8.GetBytes(msg);
        ws.SendAsync(new ArraySegment<byte>(bytes), WebSocketMessageType.Text, true, CancellationToken.None).Wait();
        var buf = new byte[1 << 16];
        while (true) {
          var sb = new StringBuilder(); WebSocketReceiveResult r;
          do { r = ws.ReceiveAsync(new ArraySegment<byte>(buf), CancellationToken.None).Result; sb.Append(Encoding.UTF8.GetString(buf, 0, r.Count)); } while (!r.EndOfMessage);
          string s = sb.ToString();
          if (!s.StartsWith("{\"id\":" + me + ",")) continue;
          var v = Regex.Match(s, "\"value\":(.*)}}}$");
          return v.Success ? v.Groups[1].Value : null;
        }
      }
    }
    public int Frame() {
      string f = Eval("document.getElementById('frame').textContent");
      if (f == null) return -1;
      var m = Regex.Match(f, "\\d+"); return m.Success ? int.Parse(m.Value) : -1;
    }
    static string Quote(string s) { return "\"" + s.Replace("\\", "\\\\").Replace("\"", "\\\"") + "\""; }
    public void Dispose() { try { ws.Abort(); } catch { } }
  }

  // ---- watching while something happens ---------------------------------------------------

  // Every 50 ms: how long the window takes to answer, whether Windows calls it hung, and the CPU
  // the application's processes used since the last look (100 = one core). With a page, the frame
  // number it shows; with a viewer rectangle, a photograph every 100 ms and a look at the viewer.
  public class Watch {
    public IntPtr h; public List<int> pids; public Page page; public Rectangle viewer = Rectangle.Empty;
    public List<double[]> pings = new List<double[]>();   // t, answer ms (negative = gave up), hung, cpu %
    public List<double[]> frames = new List<double[]>();  // t, frame
    public List<long[]> looks = new List<long[]>();       // t, hash of the viewer, blank (whole client), blank (viewer)
    public List<Bitmap> keep = new List<Bitmap>(); public int keepEvery = 0, keepMax = 6;
    Thread pinger, framer, camera; volatile bool stop;
    public void Start() {
      stop = false;
      pinger = new Thread(() => {
        double lastCpu = Cpu(pids), lastT = Now();
        while (!stop) {
          double t = Now();
          double answer = Ping(h, 10000);
          bool hung = IsHungAppWindow(h);
          double c = Cpu(pids), tt = Now();
          double pct = (c - lastCpu) / Math.Max(1, tt - lastT) * 100; lastCpu = c; lastT = tt;
          lock (pings) pings.Add(new double[] { t, answer, hung ? 1 : 0, pct });
          double wait = 50 - (Now() - t); if (wait > 0) Thread.Sleep((int)wait);
        }
      });
      pinger.Start();
      if (page != null) {
        framer = new Thread(() => {
          while (!stop) {
            double t = Now(); int n = -1; try { n = page.Frame(); } catch { }
            lock (frames) frames.Add(new double[] { t, n });
            double wait = 50 - (Now() - t); if (wait > 0) Thread.Sleep((int)wait);
          }
        });
        framer.Start();
      }
      if (!viewer.IsEmpty) {
        camera = new Thread(() => {
          int i = 0;
          while (!stop) {
            double t = Now();
            using (var b = Grab(Client(h))) {
              var whole = Look(b, All(b)); var v = Look(b, viewer);
              lock (looks) looks.Add(new long[] { (long)t, v[0], Blank(whole) ? 1 : 0, Blank(v) ? 1 : 0 });
              if (keepEvery > 0 && i % keepEvery == 0 && keep.Count < keepMax) keep.Add((Bitmap)b.Clone());
            }
            i++;
            double wait = 100 - (Now() - t); if (wait > 0) Thread.Sleep((int)wait);
          }
        });
        camera.Start();
      }
    }
    public void Stop() { stop = true; pinger.Join(); if (framer != null) framer.Join(); if (camera != null) camera.Join(); }
  }

  // ---- start-up ----------------------------------------------------------------------------

  // Starts the program and watches it come up. Every time is in ms from the moment it was asked
  // to start: the window existing, the window visible, the first photograph that is not blank, and
  // the last change before the picture stayed the same for `still` ms.
  public class Launch {
    public double T0, exists = -1, visible = -1, nonBlank = -1, settled = -1, page = -1, picture = -1;
    public Process proc; public int pid; public IntPtr h; public string error, viewRect; public List<int> tree;
    public long[] memory; public Bitmap last;
    public void Start(string exe, string args, int port) {
      var psi = new ProcessStartInfo(exe, args) { UseShellExecute = false };
      if (port > 0) psi.EnvironmentVariables["WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS"] = "--remote-debugging-port=" + port;
      T0 = Now();
      proc = Process.Start(psi); pid = proc.Id;
      while (Now() - T0 < 30000) {
        IntPtr w = Window(pid);
        if (w != IntPtr.Zero) { if (exists < 0) exists = Now() - T0; h = w; if (IsWindowVisible(w)) { visible = Now() - T0; break; } }
        Thread.Sleep(1);
      }
      if (visible < 0) error = "the window never became visible";
    }
    // Photographs the window until `limit` ms after the start. With `whole`, it watches all that
    // time and `settled` is the last change, provided nothing changed in the final `still` ms: a
    // pause while a project is still loading is not mistaken for the end. Without it, it stops at
    // the first `still` ms without a change, which is enough to know the window came up.
    public void Watch(double still, double limit, bool whole) {
      if (error != null) return;
      if (!Front(h)) { error = "the window would not come to the front"; return; }
      long lastHash = 0; double sameSince = -1;
      while (Now() - T0 < limit) {
        double t = Now();
        IntPtr fg = GetForegroundWindow();
        if (fg != h) { error = "another window came to the front while it started (" + Who(fg) + ")"; break; }
        var b = Grab(Client(h));
        var l = Look(b, All(b));
        if (nonBlank < 0 && !Blank(l)) nonBlank = t - T0;
        if (l[0] != lastHash || last == null) { lastHash = l[0]; sameSince = t - T0; if (last != null) last.Dispose(); last = b; }
        else { b.Dispose(); if (!whole && nonBlank >= 0 && t - T0 - sameSince >= still) { settled = sameSince; break; } }
      }
      if (whole && error == null && nonBlank >= 0 && limit - sameSince >= still) settled = sameSince;
      if (error == null && settled < 0) error = "the picture never stopped changing";
      tree = Tree(pid); memory = Memory(tree);
    }
    // With the debugging port: when the port answered, and when the page had its first picture
    // (the drawn-on-the-card mark set, or any pixel in the viewer canvas). Then, 1.5 s later,
    // the viewer canvas's size and place on the page.
    public void WatchPage(int port, double limit) {
      if (error != null) return;
      var p = Page.Open(port, limit);
      if (p == null) { error = "the debugging port never answered"; return; }
      page = Now() - T0;
      const string drawn = "document.documentElement.classList.contains('onscreen') || (() => { const c = document.getElementById('view'); const d = c.getContext('2d').getImageData(0, 0, c.width, c.height).data; for (let i = 3; i < d.length; i += 4) if (d[i]) return true; return false; })()";
      using (p) {
        while (Now() - T0 < limit) { if (p.Eval(drawn) == "true") { picture = Now() - T0; break; } Thread.Sleep(5); }
        Thread.Sleep(1500);
        viewRect = p.Eval("JSON.stringify(document.getElementById('view').getBoundingClientRect())");
      }
      if (picture < 0) error = "no picture arrived";
      tree = Tree(pid);
    }
    // WM_CLOSE, as the close button and the rebuild watcher send it. Returns [ms to exit, exit
    // code (-999: still running after 15 s, then stopped), processes of the tree still running 5 s
    // after the close was asked for].
    public double[] Close() {
      if (tree == null) tree = Tree(pid);
      double t0 = Now();
      PostMessage(h, 0x10, IntPtr.Zero, IntPtr.Zero);
      bool exited = proc.WaitForExit(15000);
      double ms = Now() - t0;
      double code = exited ? proc.ExitCode : -999;
      double wait = 5000 - (Now() - t0); if (wait > 0) Thread.Sleep((int)wait);
      int strays = 0; foreach (int p in tree) if (Alive(p)) strays++;
      if (!exited || strays > 0) { KillTree(tree); Thread.Sleep(1000); }
      if (last != null) { last.Dispose(); last = null; }
      return new double[] { exited ? ms : -1, code, strays };
    }
  }
}
'@

[void][H]::SetProcessDPIAware()
[void][H]::timeBeginPeriod(1)
$port = 9333
$runStart = Get-Date

# ---- the projects ------------------------------------------------------------------------------

# Copies beside copied drawings: a project opens with its own folder as the media root, so the
# reference shot's file in verification/ opened where it lies would show every drawing missing.
$work = Join-Path $env:TEMP ('p24_' + [guid]::NewGuid().ToString('N').Substring(0, 8))
New-Item -ItemType Directory $work | Out-Null
foreach ($l in 'layer1', 'layer2', 'layer3', 'layer4') { Copy-Item -Recurse (Join-Path $root "Fixtures\reference_shot\$l") $work }
$text = [IO.File]::ReadAllText((Join-Path $root 'verification\B-08a_project.json'))
$reference = Join-Path $work 'reference.json'
[IO.File]::WriteAllText($reference, $text)
# The heavy project: the reference shot with ten of the fourth batch's card effects (B-151's
# settings) and a drop shadow spread over its four layers, every one drawn on the card.
$fx = @(
  '{"instance_id":"p24-a","type_id":"core.median","enabled":true,"parameters":{"radius":4,"operate_on_alpha":"on"}},{"instance_id":"p24-b","type_id":"core.smart_blur","enabled":true,"parameters":{"radius":4,"threshold":40}},{"instance_id":"p24-c","type_id":"core.cell_pattern","enabled":true,"parameters":{"pattern":"crystals","invert":"off","contrast":120,"disperse":0.8,"size":30,"evolution":20,"seed":2,"opacity":60,"dark_color":"#102040","light_color":"#f0e0c0","blend":"screen"}}',
  '{"instance_id":"p24-d","type_id":"core.roughen_edges","enabled":true,"parameters":{"edge_type":"roughen_color","edge_color":"#8a3c14","border":6,"size":8,"complexity":3,"evolution":30,"speed":10,"seed":3}},{"instance_id":"p24-e","type_id":"core.radial_shadow","enabled":true,"parameters":{"color":"#203040","opacity":70,"light":[30,10],"distance":15,"softness":8,"render":"glass_edge","color_influence":60,"shadow_only":"off"}},{"instance_id":"p24-f","type_id":"core.bevel_alpha","enabled":true,"parameters":{"edge_thickness":4,"light_angle":-45,"light_color":"#fff0d0","light_intensity":0.6}}',
  '{"instance_id":"p24-g","type_id":"core.snowfall","enabled":true,"parameters":{"color":"#ffffff","density":60,"spacing":40,"size":5,"depth":40,"speed":3,"wind":0.5,"wiggle":3,"period":48,"seed":5,"opacity":90}},{"instance_id":"p24-h","type_id":"core.optics_compensation","enabled":true,"parameters":{"field_of_view":60,"reverse":"off","orientation":"diagonal","center":[45,55]}}',
  '{"instance_id":"p24-i","type_id":"core.polar_coordinates","enabled":true,"parameters":{"interpolation":70,"conversion":"rect_to_polar"}},{"instance_id":"p24-j","type_id":"core.corner_pin","enabled":true,"parameters":{"upper_left":[5,3],"upper_right":[92,8],"lower_left":[0,100],"lower_right":[110,96]}},{"instance_id":"p24-k","type_id":"core.drop_shadow","enabled":true,"parameters":{"color":"#000000","opacity":60,"direction":200,"distance":6,"softness":4}}'
)
$script:fxAt = 0
$heavyText = [regex]::Replace($text, '"effects": \[\]', { param($m) $r = '"effects": [' + $fx[$script:fxAt] + ']'; $script:fxAt++; $r })
if ($script:fxAt -ne 4) { throw "expected four layers to give effects to, found $($script:fxAt)" }
$heavy = Join-Path $work 'heavy.json'
[IO.File]::WriteAllText($heavy, $heavyText)
$cases = [ordered]@{
  none      = @{ args = ''; name = 'No file given (opens the built-in reference shot)' }
  reference = @{ args = "`"$reference`""; name = 'The reference shot, opened from its file' }
  heavy     = @{ args = "`"$heavy`""; name = 'Heavy: the reference shot with 11 card effects' }
}

# ---- what is on disk before, so what the runs leave behind can be found -------------------------

$appData = Join-Path $env:APPDATA 'dev.anitastic.anime-compositor'
$localData = Join-Path $env:LOCALAPPDATA 'dev.anitastic.anime-compositor'
$recent = Join-Path $appData 'recent.txt'
$recentBytes = if (Test-Path $recent) { [IO.File]::ReadAllBytes($recent) } else { $null }
# Where the program remembers its window (B-174), and how: "x y width height normal|maximized",
# the outer corner and the inner size, in pixels.
$windowFile = Join-Path $appData 'window.txt'
$windowBytes = if (Test-Path $windowFile) { [IO.File]::ReadAllBytes($windowFile) } else { $null }
function Forget-Window { if (Test-Path $windowFile) { Remove-Item -Force $windowFile } }
function Files {
  # Everything under the program's own folders (apart from the web view's profile, which is its
  # cache), verification/ and the reference shot's drawings, and the repository's top folder.
  $deep = @($appData, $localData, (Join-Path $root 'verification'), (Join-Path $root 'Fixtures\reference_shot')) | Where-Object { Test-Path $_ }
  @($deep | ForEach-Object { Get-ChildItem -Recurse -File $_ -ErrorAction SilentlyContinue | Where-Object { $_.FullName -notmatch '\\EBWebView\\' } | ForEach-Object { $_.FullName } }) +
  @(Get-ChildItem -File $root | ForEach-Object { $_.FullName })
}
$filesBefore = @(Files)

# ---- small helpers -----------------------------------------------------------------------------

function Med($xs) { $s = @($xs | Where-Object { $null -ne $_ } | Sort-Object); if ($s.Count -eq 0) { return $null }; if ($s.Count % 2) { $s[[int][math]::Floor($s.Count / 2)] } else { ($s[$s.Count / 2 - 1] + $s[$s.Count / 2]) / 2 } }
function Worst($xs) { (@($xs | Where-Object { $null -ne $_ }) | Measure-Object -Maximum).Maximum }
function Ms($x) { if ($null -eq $x -or $x -lt 0) { 'n/a' } else { '{0:N0} ms' -f $x } }
function MiB($x) { if ($null -eq $x) { 'n/a' } else { '{0:N0} MiB' -f ($x / 1MB) } }
function Machine-Load {
  # Total processor use over two seconds, and the three busiest programs other than this one.
  $a = @{}; Get-Process | ForEach-Object { try { $a[$_.Id] = $_.TotalProcessorTime.TotalMilliseconds } catch {} }
  $c = (Get-Counter '\Processor(_Total)\% Processor Time' -SampleInterval 2 -MaxSamples 1).CounterSamples[0].CookedValue
  $top = Get-Process | ForEach-Object { try { [pscustomobject]@{ n = $_.Name; c = ($_.TotalProcessorTime.TotalMilliseconds - $a[$_.Id]) / 2000 } } catch {} } |
    Where-Object { $_.n -ne 'powershell' -and $_.n -ne 'Idle' } | Sort-Object c -Descending | Select-Object -First 3
  '{0:N0}% of the whole processor; busiest: {1}' -f $c, (($top | ForEach-Object { '{0} {1:N2} cores' -f $_.n, $_.c }) -join ', ')
}
function Wait-Idle {
  # The person first: no press until nobody has touched the mouse or keyboard for two seconds.
  $t = Get-Date
  while ([H]::IdleMs() -lt 2000) { if (((Get-Date) - $t).TotalSeconds -gt 120) { return $false }; Start-Sleep -Milliseconds 200 }
  $true
}
function Save($bmp, $name) { $bmp.Save((Join-Path $out "$Tag`_$name.png"), [Drawing.Imaging.ImageFormat]::Png); "$Tag`_$name.png" }
$where = "JSON.stringify({w: innerWidth, h: innerHeight, dpr: devicePixelRatio, view: document.getElementById('view').getBoundingClientRect(), stage: document.getElementById('stage').getBoundingClientRect()})"
function Layout($page) { $page.Eval($where) | ConvertFrom-Json }
function Viewer($page) {
  # The viewer canvas as a rectangle of the client area's photograph.
  $j = ($page.Eval($where) | ConvertFrom-Json) | ConvertFrom-Json
  New-Object Drawing.Rectangle ([int]($j.view.x * $j.dpr)), ([int]($j.view.y * $j.dpr)), ([int]($j.view.width * $j.dpr)), ([int]($j.view.height * $j.dpr))
}
function Summarise-Watch($w) {
  $p = @($w.pings)
  [pscustomobject]@{
    longest = Worst ($p | ForEach-Object { [math]::Abs($_[1]) })
    gaveUp = @($p | Where-Object { $_[1] -lt 0 }).Count
    hung = @($p | Where-Object { $_[2] -gt 0 }).Count
    cpuMean = ($p | ForEach-Object { $_[3] } | Measure-Object -Average).Average
    cpuMax = Worst ($p | ForEach-Object { $_[3] })
    blank = @($w.looks | Where-Object { $_[2] -gt 0 }).Count
    blankViewer = @($w.looks | Where-Object { $_[3] -gt 0 }).Count
    photos = @($w.looks).Count
  }
}
# The longest stretch, in ms, over which a list of [t, value] did not change.
function Longest-Still($rows) {
  $r = @($rows); if ($r.Count -lt 2) { return $null }
  $best = 0; $since = $r[0][0]
  for ($k = 1; $k -lt $r.Count; $k++) { if ($r[$k][1] -ne $r[$k - 1][1]) { $since = $r[$k][0] } elseif ($r[$k][0] - $since -gt $best) { $best = $r[$k][0] - $since } }
  $best
}

$md = New-Object Collections.Generic.List[string]
$summary = [ordered]@{}
$photos = New-Object Collections.Generic.List[string]
$loads = [ordered]@{}
$loads['start of the run'] = Machine-Load

# The window, placed where the drag and resize parts expect it: 2400 by 1500 pixels (1600 by 1000
# points at 150%) on the main display, big enough for the viewer to show a picture.
$placeX = 300; $placeY = 250; $placeW = 2400; $placeH = 1500

# ---- 1. start-up -------------------------------------------------------------------------------

if (Runs 'startup') {
  $loads['before start-up'] = Machine-Load
  $runs = @{}; foreach ($k in $cases.Keys) { $runs[$k] = New-Object Collections.Generic.List[object]; $runs["$k-page"] = New-Object Collections.Generic.List[object] }
  $keys = @($cases.Keys)
  for ($r = 0; $r -lt $Launches; $r++) {
    # Alternating rounds: each round starts the three cases in a different order.
    for ($c = 0; $c -lt 3; $c++) {
      $k = $keys[($r + $c) % 3]
      Forget-Window
      $l = New-Object H+Launch
      $l.Start($exe, $cases[$k].args, 0)
      $l.Watch(500, 15000, $true)
      if ($r -eq 0 -and $l.last) { $photos.Add((Save $l.last "startup_$k")) }
      $close = $l.Close()
      $runs[$k].Add([pscustomobject]@{ round = $r + 1; exists = $l.exists; visible = $l.visible; nonBlank = $l.nonBlank; settled = $l.settled; ws = $(if ($l.memory) { $l.memory[0] }); priv = $(if ($l.memory) { $l.memory[1] }); procs = $l.tree.Count; error = $l.error; close = $close[0]; code = $close[1]; strays = $close[2] })
    }
    for ($c = 0; $c -lt 3; $c++) {
      $k = $keys[($r + $c) % 3]
      Forget-Window
      $l = New-Object H+Launch
      $l.Start($exe, $cases[$k].args, $port)
      $l.WatchPage($port, 30000)
      $rect = [H]::Rect($l.h); $opened = if ([H]::IsZoomed($l.h)) { 'maximized' } else { "$($rect.R - $rect.L) by $($rect.B - $rect.T) pixels" }
      $close = $l.Close()
      $runs["$k-page"].Add([pscustomobject]@{ round = $r + 1; visible = $l.visible; page = $l.page; picture = $l.picture; view = $l.viewRect; opened = $opened; error = $l.error; close = $close[0]; code = $close[1]; strays = $close[2] })
    }
  }
  $md.Add('## 1. Start-up')
  $md.Add('')
  $md.Add("$Launches launches of each case for the four stages, in alternating rounds (each round starts the three cases in a different order), and $Launches more of each with the debugging port for the first picture. Every time is from the moment the program was asked to start. Each launch is photographed continuously for its first 15 s; **settled** is the last time the picture changed in that watch (a pause while a project is still loading does not count, and a launch still changing in its last 500 ms would read as never settling). **First** is round 1, the first launch of that case in this run; the median and the worst are over all $Launches. A true cold start (after a restart of the computer, with nothing in Windows' file cache) was not measured: a script cannot make one without restarting the machine, and the program had already been started minutes before. Memory is the program plus every web view process it started, read once its picture had settled. The first picture is when the page had one to show (read through the debugging port, so those launches are separate); the port answering is itself a few hundred ms after the window appears, so a picture earlier than that would read as that moment.")
  $md.Add('')
  $md.Add('| Case | Stage | First | Median | Worst |')
  $md.Add('|---|---|---|---|---|')
  foreach ($k in $keys) {
    $ok = @($runs[$k] | Where-Object { -not $_.error })
    $first = $runs[$k][0]
    foreach ($s in @(@('exists', 'Window exists'), @('visible', 'Window visible'), @('nonBlank', 'First photograph that is not blank'), @('settled', 'Picture settled (its last change in the first 15 s)'))) {
      $md.Add("| $($cases[$k].name) | $($s[1]) | $(Ms $first.($s[0])) | $(Ms (Med ($ok | ForEach-Object { $_.($s[0]) }))) | $(Ms (Worst ($ok | ForEach-Object { $_.($s[0]) }))) |")
    }
    $pg = @($runs["$k-page"] | Where-Object { -not $_.error })
    $md.Add("| $($cases[$k].name) | Debugging port answered (port launches) | $(Ms $runs["$k-page"][0].page) | $(Ms (Med ($pg | ForEach-Object { $_.page }))) | $(Ms (Worst ($pg | ForEach-Object { $_.page }))) |")
    $md.Add("| $($cases[$k].name) | First picture in the viewer (port launches) | $(Ms $runs["$k-page"][0].picture) | $(Ms (Med ($pg | ForEach-Object { $_.picture }))) | $(Ms (Worst ($pg | ForEach-Object { $_.picture }))) |")
    $md.Add("| $($cases[$k].name) | Memory at settled: working set | $(MiB $first.ws) | $(MiB (Med ($ok | ForEach-Object { $_.ws }))) | $(MiB (Worst ($ok | ForEach-Object { $_.ws }))) |")
    $md.Add("| $($cases[$k].name) | Memory at settled: private bytes | $(MiB $first.priv) | $(MiB (Med ($ok | ForEach-Object { $_.priv }))) | $(MiB (Worst ($ok | ForEach-Object { $_.priv }))) |")
    $md.Add("| $($cases[$k].name) | Processes (program + web view) | $($first.procs) | $(Med ($ok | ForEach-Object { $_.procs })) | $(Worst ($ok | ForEach-Object { $_.procs })) |")
    $md.Add("| $($cases[$k].name) | Close (WM_CLOSE to the program ending) | $(Ms $first.close) | $(Ms (Med ($ok | ForEach-Object { $_.close }))) | $(Ms (Worst ($ok | ForEach-Object { $_.close }))) |")
  }
  $md.Add('')
  $all = @($keys | ForEach-Object { $runs[$_]; $runs["$_-page"] })
  $failed = @($all | Where-Object { $_.error })
  $unclean = @($all | Where-Object { $_.code -ne 0 })
  $strayed = @($all | Where-Object { $_.strays -gt 0 })
  $md.Add("Launches: $($all.Count). Did not come up as measured: $($failed.Count)$(if ($failed.Count) { ' (' + (($failed | ForEach-Object { "round $($_.round): $($_.error)" }) -join '; ') + ')' }). Exit code other than 0: $($unclean.Count). Left a process running 5 s after closing: $($strayed.Count).")
  $md.Add('')
  $views = @($all | Where-Object { $_.view } | ForEach-Object { $j = ($_.view | ConvertFrom-Json) | ConvertFrom-Json; '{0:N0} by {1:N0} points at {2:N0},{3:N0}' -f $j.width, $j.height, $j.x, $j.y } | Sort-Object -Unique)
  $openedAs = @($all | Where-Object { $_.opened } | ForEach-Object { $_.opened } | Sort-Object -Unique)
  $md.Add("Every launch began with no remembered window, as a first launch does. The window opened: $($openedAs -join '; '). The viewer canvas at that size, 1.5 s after the first picture: $($views -join '; ').")
  $md.Add('')
  $md.Add('The photographs are round 1 of each case at its settled moment: ' + (($photos | Where-Object { $_ -like '*startup*' } | ForEach-Object { "``$_``" }) -join ', ') + '.')
  $md.Add('')
  $summary['Start-up: every launch came up, settled and closed cleanly'] = if ($failed.Count -or $unclean.Count -or $strayed.Count) { "FAIL ($($failed.Count) did not come up, $($unclean.Count) unclean exits, $($strayed.Count) left processes, of $($all.Count))" } else { "PASS ($($all.Count) of $($all.Count))" }
  $summary['Start-up to settled picture, median (no file / reference / heavy)'] = (($keys | ForEach-Object { Ms (Med (@($runs[$_] | Where-Object { -not $_.error }) | ForEach-Object { $_.settled })) }) -join ' / ') + ' (no target exists; measured only)'
  $small = @($all | Where-Object { $_.view } | Where-Object { (($_.view | ConvertFrom-Json) | ConvertFrom-Json).height -lt 100 })
  $summary['The viewer has room for the picture at the size the window opens at'] = if ($small.Count) { "FAIL ($($views -join '; '))" } else { "PASS ($($views -join '; '))" }
}

# ---- 1b. remembering the window (B-174) ---------------------------------------------------------

function Open-Plain {
  $l = New-Object H+Launch
  $l.Start($exe, '', 0)
  $l.Watch(500, 15000, $false)
  $l
}
function Window-State($h) {
  $r = [H]::Rect($h)
  [pscustomobject]@{ zoomed = [H]::IsZoomed($h); onScreen = [H]::OnScreen($h); l = $r.L; t = $r.T; w = $r.R - $r.L; hh = $r.B - $r.T
    text = $(if ([H]::IsZoomed($h)) { 'maximized' } else { "$($r.R - $r.L) by $($r.B - $r.T) pixels at $($r.L),$($r.T)" }) + $(if (-not [H]::OnScreen($h)) { ', OFF SCREEN' }) }
}
function Is-Placed($s) { -not $s.zoomed -and [math]::Abs($s.l - $placeX) -le 2 -and [math]::Abs($s.t - $placeY) -le 2 -and [math]::Abs($s.w - $placeW) -le 2 -and [math]::Abs($s.hh - $placeH) -le 2 }
function Sys-Command($h, $cmd, $test) {
  [void][H]::PostMessage($h, 0x112, [IntPtr]$cmd, [IntPtr]::Zero)
  $t = Get-Date; while (-not (& $test) -and ((Get-Date) - $t).TotalSeconds -lt 5) { Start-Sleep -Milliseconds 50 }
  Start-Sleep -Milliseconds 700
}

if (Runs 'remember') {
  $loads['before the remembered-window part'] = Machine-Load
  $rem = New-Object Collections.Generic.List[object]
  function Remember-Row($what, $expected, $l, $test, $photoName) {
    if ($l.error) { $rem.Add([pscustomobject]@{ what = $what; expected = $expected; got = "did not come up: $($l.error)"; ok = $false; photo = $null }); return }
    $s = Window-State $l.h
    [void][H]::Settle($l.h, [H]::Now(), 300, 3000)
    $p = Save ([H]::Settled) $photoName; $photos.Add($p)
    $rem.Add([pscustomobject]@{ what = $what; expected = $expected; got = $s.text; ok = [bool](& $test $s); photo = $p })
  }
  $rememberCloses = New-Object Collections.Generic.List[object]
  function Close-Counted($l) { $c = $l.Close(); $rememberCloses.Add($c) }

  # A first launch: nothing remembered.
  Forget-Window
  $l = Open-Plain
  Remember-Row 'First launch (no remembered window)' 'maximized' $l { param($s) $s.zoomed -and $s.onScreen } 'remember_first_launch'
  Close-Counted $l

  # Placed by hand, closed, started again.
  $l = Open-Plain
  if (-not $l.error) { [H]::Place($l.h, $placeX, $placeY, $placeW, $placeH); Start-Sleep -Milliseconds 700 }
  Close-Counted $l
  $l = Open-Plain
  Remember-Row "Closed at $placeW by $placeH pixels at $placeX,$placeY, started again" "$placeW by $placeH pixels at $placeX,$placeY" $l { param($s) Is-Placed $s } 'remember_size'
  # Maximized, closed, started again; then Restore.
  if (-not $l.error) { $h = $l.h; Sys-Command $h 0xF030 { [H]::IsZoomed($h) } }
  Close-Counted $l
  $l = Open-Plain
  Remember-Row 'Closed maximized, started again' 'maximized' $l { param($s) $s.zoomed -and $s.onScreen } 'remember_maximized'
  if (-not $l.error) {
    $h = $l.h; Sys-Command $h 0xF120 { -not [H]::IsZoomed($h) }
    Remember-Row '... then Restore pressed' "back to $placeW by $placeH pixels at $placeX,$placeY" $l { param($s) Is-Placed $s } 'remember_restored'
  }
  Close-Counted $l

  # A remembered place on a display that is no longer attached.
  [IO.File]::WriteAllText($windowFile, "20000 20000 1600 1000 normal`n")
  $l = Open-Plain
  Remember-Row 'Remembered at 20000,20000 (no display there)' 'on screen' $l { param($s) $s.onScreen } 'remember_off_screen'
  Close-Counted $l

  # A damaged file.
  [IO.File]::WriteAllText($windowFile, "this is not a window`n")
  $l = Open-Plain
  Remember-Row 'Remembered-window file damaged' 'maximized, as a first launch' $l { param($s) $s.zoomed -and $s.onScreen } 'remember_damaged'
  Close-Counted $l
  Forget-Window

  $md.Add('## 1b. Remembering the window')
  $md.Add('')
  $md.Add("No file given, no debugging port. Each row starts the program, waits for its picture to settle and reads where Windows put the window; the steps run in this order, each closing the window (WM_CLOSE) before the next start. Placing the window is SetWindowPos, as dragging it ends; maximize and Restore are sent as the title-bar buttons send them. Where the program remembers its window: ``%APPDATA%\dev.anitastic.anime-compositor\window.txt``, written here by hand for the last two rows.")
  $md.Add('')
  $md.Add('| Step | Expected | Window | Result | Photograph |')
  $md.Add('|---|---|---|---|---|')
  foreach ($x in $rem) { $md.Add("| $($x.what) | $($x.expected) | $($x.got) | $(if ($x.ok) { 'PASS' } else { 'FAIL' }) | $(if ($x.photo) { '`' + $x.photo + '`' }) |") }
  $md.Add('')
  $md.Add('Closing these windows: ' + (($rememberCloses | ForEach-Object { "exit code $($_[1]), $($_[2]) left" }) -join '; ') + '.')
  $md.Add('')
  foreach ($x in $rem) { $summary["Remembered window: $($x.what)"] = if ($x.ok) { "PASS ($($x.got))" } else { "FAIL ($($x.got); expected $($x.expected))" } }
  $summary['Remembered window: every launch closed cleanly'] = if (@($rememberCloses | Where-Object { $_[1] -ne 0 -or $_[2] -gt 0 }).Count) { 'FAIL' } else { "PASS ($($rememberCloses.Count) of $($rememberCloses.Count))" }
}

# ---- 2-4. dragging, resizing, maximizing --------------------------------------------------------

function Open-Placed($given) {
  $l = New-Object H+Launch
  $l.Start($exe, $given, $port)
  $l.Watch(500, 15000, $true)
  if ($l.error) { throw "start: $($l.error)" }
  [H]::Place($l.h, $placeX, $placeY, $placeW, $placeH)
  [void][H]::Front($l.h)
  $page = [H+Page]::Open($port, 10000)
  if (-not $page) { throw 'the debugging port never answered' }
  [void][H]::Settle($l.h, [H]::Now(), 800, 20000)
  @{ l = $l; page = $page }
}
function Path-Caption($dx, $dy) {
  # Three seconds, a step every 10 ms: an arc that ends ($dx, $dy) from where it began, smooth
  # enough that Windows' shake-to-minimize never sees a shake.
  $n = 300; $xs = New-Object int[] $n; $ys = New-Object int[] $n
  for ($k = 0; $k -lt $n; $k++) { $t = ($k + 1) / $n; $xs[$k] = [int]($dx * $t + 220 * [math]::Sin([math]::PI * $t)); $ys[$k] = [int]($dy * $t - 120 * [math]::Sin([math]::PI * $t)) }
  @{ x = $xs; y = $ys }
}
function Path-Corner {
  # Forty steps of 75 ms: out to 300 by 200 pixels bigger, back, in to 300 by 200 smaller, back.
  $xs = New-Object int[] 40; $ys = New-Object int[] 40
  for ($k = 0; $k -lt 40; $k++) { $q = ($k + 1) / 10; $f = if ($q -le 1) { $q } elseif ($q -le 3) { 2 - $q } else { $q - 4 }; $xs[$k] = [int](300 * $f); $ys[$k] = [int](200 * $f) }
  @{ x = $xs; y = $ys }
}
function Drag-Run($o, $what, $playing, $photoName) {
  # One drag of the title bar (or of the corner) with the watch running around it.
  $l = $o.l; $page = $o.page
  if (-not (Wait-Idle)) { return [pscustomobject]@{ what = $what; skipped = 'somebody was using the mouse or keyboard' } }
  [void][H]::Front($l.h)
  $viewer = Viewer $page
  $before = [H]::Rect($l.h); $layoutBefore = Layout $page
  [void][H]::Settle($l.h, [H]::Now(), 300, 3000)
  $picBefore = [H]::Settled.Clone()
  $corner = $what -like '*corner*'
  if ($corner) { $pt = [H]::Corner($l.h); $path = Path-Corner; $hit = 17; $step = 75; $mx = 0; $my = 0 } else { $pt = [H]::Caption($l.h); $path = Path-Caption 200 120; $hit = 2; $step = 10; $mx = 200; $my = 120 }
  if ($pt.X -lt 0) { return [pscustomobject]@{ what = $what; skipped = 'no point Windows called the ' + $(if ($corner) { 'corner' } else { 'title bar' }) } }
  $w = New-Object H+Watch; $w.h = $l.h; $w.pids = $l.tree; $w.page = $page; $w.viewer = $viewer; $w.keepEvery = 6
  $w.Start()
  Start-Sleep -Milliseconds 600
  $t0 = [H]::Now()
  $err = [H]::Drag($l.h, $pt.X, $pt.Y, $path.x, $path.y, $step, $hit)
  $t1 = [H]::Now()
  Start-Sleep -Milliseconds 1500
  $w.Stop()
  if ($err) { return [pscustomobject]@{ what = $what; skipped = $err } }
  $after = [H]::Rect($l.h)
  $placed = ([math]::Abs($after.L - $before.L - $mx) -le 2) -and ([math]::Abs($after.T - $before.T - $my) -le 2) -and ([math]::Abs(($after.R - $after.L) - ($before.R - $before.L)) -le 2) -and ([math]::Abs(($after.B - $after.T) - ($before.B - $before.T)) -le 2)
  [void][H]::Settle($l.h, [H]::Now(), $(if ($playing) { 0 } else { 500 }), 5000)
  $look = [H]::Look([H]::Settled, [H]::All([H]::Settled))
  $layoutAfter = Layout $page
  $photo = Save ([H]::Settled) $photoName
  $photos.Add($photo)
  # Photographs during: three, spread over the drag, and only while playing (an idle window shows nothing new).
  $kept = @(); $n = 0; foreach ($b in $w.keep) { $n++; if ($playing -and $n % 2 -eq 0) { $kept += (Save $b "$photoName`_during_$($n / 2)") }; $b.Dispose() }
  foreach ($p in $kept) { $photos.Add($p) }
  $s = Summarise-Watch $w
  # Playback: the page's frame against the frame its clock should have reached. The page loops by
  # the wall clock (preview.rs), so from the last frame read before the press, 24 frames a second.
  $fr = @($w.frames | Where-Object { $_[1] -ge 0 })
  $during = @($fr | Where-Object { $_[0] -ge $t0 -and $_[0] -le $t1 })
  $behind = $null; $distinct = $null; $stillMs = $null
  if ($playing -and $fr.Count) {
    $a = @($fr | Where-Object { $_[0] -lt $t0 }) | Select-Object -Last 1
    $z = @($fr | Where-Object { $_[0] -ge $t1 + 1000 }) | Select-Object -First 1
    if ($a -and $z) {
      $expected = ($a[1] + [math]::Round(($z[0] - $a[0]) / 1000 * 24)) % 240
      $behind = (($expected - $z[1]) % 240 + 240 + 120) % 240 - 120
    }
    $distinct = @($during | ForEach-Object { $_[1] } | Sort-Object -Unique).Count
    $stillMs = Longest-Still $during
  }
  $viewDuring = @($w.looks | Where-Object { $_[0] -ge $t0 -and $_[0] -le $t1 })
  [pscustomobject]@{
    what = $what; skipped = $null; longest = $s.longest; gaveUp = $s.gaveUp; hung = $s.hung; cpuMean = $s.cpuMean; cpuMax = $s.cpuMax
    blank = $s.blank; blankViewer = $s.blankViewer; photos = $s.photos; placed = $placed
    before = "$($before.L),$($before.T) $($before.R - $before.L)x$($before.B - $before.T)"; after = "$($after.L),$($after.T) $($after.R - $after.L)x$($after.B - $after.T)"
    sameLayout = ($layoutBefore -eq $layoutAfter); layoutBefore = $layoutBefore; layoutAfter = $layoutAfter; beforePhoto = $(if ($layoutBefore -ne $layoutAfter) { $q = Save $picBefore "$photoName`_before"; $photos.Add($q); $q }); samePicture = [H]::Diff($picBefore, [H]::Settled)
    behind = $behind; distinct = $distinct; frameStill = $stillMs
    viewerPictures = @($viewDuring | ForEach-Object { $_[1] } | Sort-Object -Unique).Count; viewerPhotos = $viewDuring.Count; viewerStill = Longest-Still $viewDuring
    photo = $photo; during = ($kept | ForEach-Object { "``$_``" }) -join ', '; ms = $t1 - $t0
  }
}
function Play($page) { [void]$page.Eval("document.getElementById('play').textContent === 'Play' && document.getElementById('play').click()"); Start-Sleep -Seconds 2 }
function Stop-Play($page) { [void]$page.Eval("document.getElementById('play').textContent !== 'Play' && document.getElementById('play').click()"); Start-Sleep -Seconds 1 }

if (Runs 'window') {
  $loads['before the window parts'] = Machine-Load
  $drags = New-Object Collections.Generic.List[object]
  $cycles2 = New-Object Collections.Generic.List[object]
  $windowCloses = New-Object Collections.Generic.List[object]
  foreach ($k in 'none', 'heavy') {
    $o = Open-Placed $cases[$k].args
    try {
      if ($k -eq 'none') {
        $drags.Add((Drag-Run $o 'Title bar, idle' $false 'drag_idle'))
        $drags.Add((Drag-Run $o 'Title bar, idle, second time' $false 'drag_idle_2'))
      }
      Play $o.page
      $drags.Add((Drag-Run $o "Title bar, playing ($($cases[$k].name))" $true "drag_playing_$k"))
      Stop-Play $o.page
      if ($k -eq 'none') {
        $drags.Add((Drag-Run $o 'Bottom-right corner, idle' $false 'resize_idle'))
        Play $o.page
        $drags.Add((Drag-Run $o 'Bottom-right corner, playing' $true 'resize_playing'))
        Stop-Play $o.page
        # Maximize and restore, minimize and restore, as the title-bar buttons ask for them.
        [void][H]::Front($o.l.h)
        [void][H]::Settle($o.l.h, [H]::Now(), 500, 5000)
        $normal = [H]::Settled.Clone(); $photos.Add((Save $normal 'before_maximize'))
        $frame0 = $o.page.Frame()
        for ($c = 1; $c -le 5; $c++) {
          foreach ($m in @(@('maximize', 0xF030), @('minimize', 0xF020))) {
            $t = [H]::Now()
            [void][H]::PostMessage($o.l.h, 0x112, [IntPtr]$m[1], [IntPtr]::Zero)
            $lookThere = $null
            if ($m[0] -eq 'maximize') {
              $there = [H]::Settle($o.l.h, $t, 500, 10000); $reached = [H]::IsZoomed($o.l.h); $lookThere = [H]::Look([H]::Settled, [H]::All([H]::Settled))
              if ($c -eq 1) { $photos.Add((Save ([H]::Settled) 'maximized')) }
            } else {
              while (-not [H]::IsIconic($o.l.h) -and [H]::Now() - $t -lt 5000) { Start-Sleep -Milliseconds 5 }
              $there = [H]::Now() - $t; $reached = [H]::IsIconic($o.l.h); Start-Sleep -Seconds 1
            }
            $t = [H]::Now()
            [void][H]::PostMessage($o.l.h, 0x112, [IntPtr]0xF120, [IntPtr]::Zero)
            [void][H]::Front($o.l.h)
            $back = [H]::Settle($o.l.h, $t, 500, 10000)
            $look = [H]::Look([H]::Settled, [H]::All([H]::Settled))
            if ($c -eq 1) { $photos.Add((Save ([H]::Settled) "restored_from_$($m[0])")) }
            $rect = [H]::Rect($o.l.h)
            $cycles2.Add([pscustomobject]@{ kind = $m[0]; n = $c; there = $there; reached = $reached; thereBlank = $(if ($lookThere) { [H]::Blank($lookThere) } else { $false }); back = $back; same = [H]::Diff($normal, [H]::Settled); blank = [H]::Blank($look); frame = $o.page.Frame(); frame0 = $frame0; rect = "$($rect.L),$($rect.T) $($rect.R - $rect.L)x$($rect.B - $rect.T)" })
            Start-Sleep -Milliseconds 500
          }
        }
      }
    } finally { $c0 = $o.l.Close(); $windowCloses.Add([pscustomobject]@{ k = $k; code = $c0[1]; strays = $c0[2] }) }
  }

  $md.Add('## 2-4. Dragging and resizing the window')
  $md.Add('')
  $md.Add("These launches use the debugging port (to read the frame number the page shows and to press Play). The window is first placed at $placeX,$placeY and made $placeW by $placeH pixels (1600 by 1000 points at 150%), so the viewer has room to show the picture. The mouse is real: SendInput presses the title bar (or the bottom-right corner) at a point Windows itself calls that, moves in small steps, and lets go. Throughout, every 50 ms: the time the window took to answer an empty message (**longest unanswered**), whether Windows called it hung (the test behind 'Not responding'), and the program's processor use (100% = one core). A photograph of the window every 100 ms is checked for blank (one or two colours, or nine tenths white) (playback is followed by the frame number instead: a photograph of a moving window can be a pixel out, so comparing them would mislead). The title-bar path is an arc of 3 s in 300 steps ending 200 right and 120 down; the corner path is 40 steps of 75 ms, out to 300 by 200 bigger, back, in to 300 by 200 smaller and back.")
  $md.Add('')
  $md.Add('| What | Longest unanswered | Hung reports | CPU mean / max | Blank photographs (window / viewer) | Ended where expected | Layout after = before | Picture after = before | Photograph after |')
  $md.Add('|---|---|---|---|---|---|---|---|---|')
  foreach ($d in $drags) {
    if ($d.skipped) { $md.Add("| $($d.what) | skipped: $($d.skipped) | | | | | | | |"); continue }
    $md.Add("| $($d.what) | $(Ms $d.longest)$(if ($d.gaveUp) { " ($($d.gaveUp) gave up at 10 s)" }) | $($d.hung) | $('{0:N0}% / {1:N0}%' -f $d.cpuMean, $d.cpuMax) | $($d.blank) / $($d.blankViewer) of $($d.photos) | $(if ($d.placed) { 'yes' } else { "NO ($($d.before) to $($d.after))" }) | $(if ($d.sameLayout) { 'yes' } else { 'NO' }) | $(if ($d.what -like '*playing*') { 'n/a (playing)' } elseif ($d.samePicture -eq 'none') { 'yes' } else { "NO ($($d.samePicture))" }) | ``$($d.photo)`` |")
  }
  $md.Add('')
  foreach ($d in $drags | Where-Object { -not $_.skipped -and -not $_.sameLayout }) {
    $md.Add("Layout of $($d.what), in points (the window's inner size, then the viewer canvas and the panel that holds it), before: ``$($d.layoutBefore)``; after: ``$($d.layoutAfter)``. Photograph before: ``$($d.beforePhoto)``.")
    $md.Add('')
  }
  $md.Add('Playback during the drags (frame numbers read from the page every 50 ms; **behind** is the frame the page showed 1 s after letting go against the frame 24 frames a second should have reached from the last frame read before pressing; positive means behind):')
  $md.Add('')
  $md.Add('| What | Different frames shown during the drag | Longest the frame number stood still during it | Frames behind 1 s after release | Photographs during |')
  $md.Add('|---|---|---|---|---|')
  foreach ($d in $drags | Where-Object { $_.what -like '*playing*' -and -not $_.skipped }) { $md.Add("| $($d.what) | $($d.distinct) in $(Ms $d.ms) | $(Ms $d.frameStill) | $($d.behind) | $($d.during) |") }
  $md.Add('')
  $md.Add('Maximize and minimize, five of each, sent as the title-bar buttons send them (WM_SYSCOMMAND), on the reference shot, idle, at the placed size. **There** is the time to the maximized picture settling (or to Windows reporting the window minimized); **back** is the time from Restore to the picture settling; **same** means the restored picture is identical, pixel for pixel, to the one before.')
  $md.Add('')
  $md.Add('| Cycle | There | Reached | Back | Restored picture same | Blank after | Frame shown (before) | Window after |')
  $md.Add('|---|---|---|---|---|---|---|---|')
  foreach ($c in $cycles2) { $md.Add("| $($c.kind) $($c.n) | $(Ms $c.there)$(if ($c.thereBlank) { ' (BLANK)' }) | $(if ($c.reached) { 'yes' } else { 'NO' }) | $(Ms $c.back) | $(if ($c.same -eq 'none') { 'yes' } else { "NO ($($c.same))" }) | $(if ($c.blank) { 'YES' } else { 'no' }) | $($c.frame) ($($c.frame0)) | $($c.rect) |") }
  $md.Add('')
  $md.Add('Closing these windows afterwards: ' + (($windowCloses | ForEach-Object { "$($_.k) exit code $($_.code), $($_.strays) processes left" }) -join '; ') + '.')
  $md.Add('')
  $ran = @($drags | Where-Object { -not $_.skipped })
  $playingDrags = @($ran | Where-Object { $_.what -like '*playing*' })
  $summary['Window parts: every drag ran (none skipped)'] = if ($ran.Count -eq $drags.Count) { "PASS ($($drags.Count) of $($drags.Count))" } else { "FAIL ($($drags.Count - $ran.Count) skipped)" }
  $summary['Never "Not responding" while dragged or resized'] = if (@($ran | Where-Object { $_.hung -gt 0 -or $_.gaveUp -gt 0 }).Count) { 'FAIL' } else { 'PASS' }
  $summary['Longest unanswered while dragged or resized'] = (Ms (Worst ($ran | ForEach-Object { $_.longest }))) + ' (no target exists; measured only)'
  $summary['No blank or white photographs while dragged or resized'] = if (@($ran | Where-Object { $_.blank -gt 0 }).Count) { 'FAIL (' + (($ran | Where-Object { $_.blank -gt 0 } | ForEach-Object { "$($_.what): $($_.blank) of $($_.photos)" }) -join '; ') + ')' } else { 'PASS' }
  $summary['Window ended where it was dragged / at its size'] = if (@($ran | Where-Object { -not $_.placed }).Count) { 'FAIL' } else { 'PASS' }
  $summary['Layout the same after dragging and resizing'] = if (@($ran | Where-Object { -not $_.sameLayout }).Count) { 'FAIL' } else { 'PASS' }
  $summary['Playback kept going while the window was dragged'] = if (@($playingDrags | Where-Object { $_.distinct -lt 2 }).Count) { 'FAIL' } else { 'PASS (frames behind 1 s after release: ' + (($playingDrags | ForEach-Object { "$($_.behind)" }) -join ', ') + ')' }
  $summary['Maximize/minimize: every restore repainted the same picture'] = if (@($cycles2 | Where-Object { $_.same -ne 'none' -or $_.blank -or -not $_.reached -or $_.thereBlank }).Count) { 'FAIL' } else { "PASS ($($cycles2.Count) of $($cycles2.Count))" }
}

# ---- 5a. launches and closes -------------------------------------------------------------------

if (Runs 'cycles') {
  $loads['before the launch-and-close cycles'] = Machine-Load
  $rows = New-Object Collections.Generic.List[object]
  for ($c = 1; $c -le $Cycles; $c++) {
    $l = New-Object H+Launch
    $l.Start($exe, '', 0)
    $l.Watch(500, 30000, $false)
    $close = $l.Close()
    $rows.Add([pscustomobject]@{ n = $c; settled = $l.settled; error = $l.error; close = $close[0]; code = $close[1]; strays = $close[2] })
  }
  $md.Add("## 5a. $Cycles launches and closes")
  $md.Add('')
  $md.Add('No file given (the built-in reference shot), no debugging port. Each launch waits for the picture to settle, then sends WM_CLOSE (what the close button and the rebuild watcher send), waits for the program to end, and 5 s after asking looks for any of its processes still running. Crash reports were searched for over the whole run (section 6).')
  $md.Add('')
  $closes = @($rows | Where-Object { $_.close -ge 0 } | ForEach-Object { $_.close })
  $other = @($rows | Where-Object { $_.code -ne 0 -and $_.code -ne -999 } | ForEach-Object { $_.code })
  $md.Add('| Launches | Came up and settled | Clean exits (code 0) | Other exit codes | Did not end within 15 s | Processes left 5 s after close | Close time, median / worst | Start to the first 500 ms without a change, median / worst |')
  $md.Add('|---|---|---|---|---|---|---|---|')
  $md.Add("| $($rows.Count) | $(@($rows | Where-Object { -not $_.error }).Count) | $(@($rows | Where-Object { $_.code -eq 0 }).Count) | $(if ($other.Count) { $other -join ', ' } else { 'none' }) | $(@($rows | Where-Object { $_.code -eq -999 }).Count) | $(($rows | Measure-Object strays -Sum).Sum) | $(Ms (Med $closes)) / $(Ms (Worst $closes)) | $(Ms (Med ($rows | ForEach-Object { $_.settled }))) / $(Ms (Worst ($rows | ForEach-Object { $_.settled }))) |")
  $md.Add('')
  $bad = @($rows | Where-Object { $_.error }); if ($bad.Count) { $md.Add('Did not come up as measured: ' + (($bad | ForEach-Object { "$($_.n): $($_.error)" }) -join '; ') + '.'); $md.Add('') }
  $summary["Launch and close $Cycles times: all clean"] = if (@($rows | Where-Object { $_.code -ne 0 -or $_.strays -gt 0 -or $_.error }).Count) { 'FAIL' } else { "PASS ($Cycles of $Cycles exit 0, nothing left running)" }
}

# ---- 5b. the soak ------------------------------------------------------------------------------

if (Runs 'soak') {
  $loads['before the soak'] = Machine-Load
  $o = Open-Placed ''
  $soak = New-Object Collections.Generic.List[object]
  $soakDrags = New-Object Collections.Generic.List[object]
  try {
    Play $o.page
    $t0 = [H]::Now(); $next = 60000; $dir = 1; $k = 0
    while ([H]::Now() - $t0 -lt $SoakMinutes * 60000) {
      $t = [H]::Now() - $t0
      if ($t -ge $k * 10000) {
        $tree = [H]::Tree($o.l.pid); $m = [H]::Memory($tree)
        $soak.Add([pscustomobject]@{ s = [int]($t / 1000); ws = $m[0]; priv = $m[1]; procs = $tree.Count; frame = $o.page.Frame(); hung = [H]::IsHungAppWindow($o.l.h) })
        $k++
      }
      if ($t -ge $next) {
        if (Wait-Idle) {
          $pt = [H]::Caption($o.l.h)
          $w = New-Object H+Watch; $w.h = $o.l.h; $w.pids = [H]::Tree($o.l.pid); $w.Start()
          $path = Path-Caption (150 * $dir) 0
          $err = if ($pt.X -ge 0) { [H]::Drag($o.l.h, $pt.X, $pt.Y, $path.x, $path.y, 10, 2) } else { 'no point Windows called the title bar' }
          Start-Sleep -Milliseconds 300; $w.Stop()
          $s = Summarise-Watch $w
          $soakDrags.Add([pscustomobject]@{ at = [int]($t / 1000); err = $err; longest = $s.longest; hung = $s.hung })
          $dir = -$dir
        } else { $soakDrags.Add([pscustomobject]@{ at = [int]($t / 1000); err = 'somebody was using the mouse or keyboard'; longest = $null; hung = 0 }) }
        $next += 60000
      }
      Start-Sleep -Milliseconds 200
    }
    $endFrames = @(1..5 | ForEach-Object { $o.page.Frame(); Start-Sleep -Milliseconds 200 })
    [void][H]::Settle($o.l.h, [H]::Now(), 0, 2000)
    $photos.Add((Save ([H]::Settled) 'soak_end'))
    Stop-Play $o.page
    $report = $o.page.Eval("document.getElementById('report').textContent")
  } finally { $soakClose = $o.l.Close() }

  $md.Add("## 5b. $SoakMinutes-minute soak")
  $md.Add('')
  $md.Add("The reference shot (no file given, debugging port) playing in a loop at the placed size for $SoakMinutes minutes, a title-bar drag of 3 s every 60 s (150 pixels right, then back), memory (program plus web view) every 10 s. One loop of the shot is 10 s, so the soak is $($SoakMinutes * 6) loops. T-06 measured the core alone, without a window, over ten loops of the same shot: 2.3 MiB of growth against a bound of 31.6 MiB.")
  $md.Add('')
  $first = $soak[0]; $last = $soak[$soak.Count - 1]
  $ten = @($soak | Where-Object { $_.s -ge 100 })[0]
  # Least-squares slope over the second half, in MiB a minute: what is still growing once the
  # first loops' one-off allocations are done.
  $half = @($soak | Where-Object { $_.s -ge $SoakMinutes * 30 })
  $mx = ($half | Measure-Object s -Average).Average
  function Slope($prop) { $my = ($half | Measure-Object $prop -Average).Average; $num = 0; $den = 0; foreach ($p in $half) { $num += ($p.s - $mx) * ($p.$prop - $my); $den += ($p.s - $mx) * ($p.s - $mx) }; if ($den) { $num / $den * 60 / 1MB } else { 0 } }
  $slope = Slope 'priv'; $slopeW = Slope 'ws'
  $md.Add('| | Working set | Private bytes |')
  $md.Add('|---|---|---|')
  $md.Add("| At the start (playing) | $(MiB $first.ws) | $(MiB $first.priv) |")
  $md.Add("| After ten loops (100 s) | $(MiB $ten.ws) | $(MiB $ten.priv) |")
  $md.Add("| Growth over the first ten loops | $(MiB ($ten.ws - $first.ws)) | $(MiB ($ten.priv - $first.priv)) |")
  $md.Add("| At the end ($($last.s) s) | $(MiB $last.ws) | $(MiB $last.priv) |")
  $md.Add("| Growth, start to end | $(MiB ($last.ws - $first.ws)) | $(MiB ($last.priv - $first.priv)) |")
  $md.Add("| Peak | $(MiB (Worst ($soak | ForEach-Object { $_.ws }))) | $(MiB (Worst ($soak | ForEach-Object { $_.priv }))) |")
  $md.Add("| Trend over the second half (straight-line fit) | $('{0:N2} MiB a minute' -f $slopeW) | $('{0:N2} MiB a minute' -f $slope) |")
  $md.Add('')
  $md.Add("Processes in the tree: $(Med ($soak | ForEach-Object { $_.procs })) (from $(($soak | Measure-Object procs -Minimum).Minimum) to $(($soak | Measure-Object procs -Maximum).Maximum)). Hung reports at the samples: $(@($soak | Where-Object { $_.hung }).Count). Frames read 200 ms apart at the end: $($endFrames -join ', '). The page's played-and-dropped sentence after stopping: $report. Closing it: exit code $($soakClose[1]), $($soakClose[2]) processes left.")
  $md.Add('')
  $md.Add('Drags during the soak (when, longest unanswered): ' + (($soakDrags | ForEach-Object { if ($_.err) { "$($_.at) s skipped ($($_.err))" } else { "$($_.at) s $(Ms $_.longest)$(if ($_.hung) { ' HUNG' })" } }) -join '; ') + '.')
  $md.Add('')
  $md.Add('Every sample (s, working set MiB, private MiB, frame): ' + (($soak | ForEach-Object { '{0} {1:N0} {2:N0} {3}' -f $_.s, ($_.ws / 1MB), ($_.priv / 1MB), $_.frame }) -join '; ') + '.')
  $md.Add('')
  $summary["Soak: growth over the first ten loops within T-06's bound (31.6 MiB)"] = if (($ten.priv - $first.priv) / 1MB -le 31.6) { "PASS ($(MiB ($ten.priv - $first.priv)) private)" } else { "FAIL ($(MiB ($ten.priv - $first.priv)) private)" }
  $summary['Soak: memory trend over the second half'] = '{0:N2} MiB a minute private, {1:N2} working set (T-06 has a bound for ten loops only; measured only)' -f $slope, $slopeW
  $summary["Soak $SoakMinutes min: still playing at the end, never hung, every drag ran"] = if (@($endFrames | Sort-Object -Unique).Count -gt 1 -and -not @($soak | Where-Object { $_.hung }).Count -and -not @($soakDrags | Where-Object { $_.hung -or $_.err }).Count) { 'PASS' } else { 'FAIL' }
}

# ---- 6. crash reports, and what the runs left behind ---------------------------------------------

$events = @(Get-WinEvent -FilterHashtable @{ LogName = 'Application'; StartTime = $runStart } -ErrorAction SilentlyContinue |
  Where-Object { $_.ProviderName -match 'Application Error|Application Hang|Windows Error Reporting' -and $_.Message -match 'anime_compositor|msedgewebview2' })
$wer = @(@((Join-Path $env:LOCALAPPDATA 'Microsoft\Windows\WER\ReportArchive'), (Join-Path $env:ProgramData 'Microsoft\Windows\WER\ReportArchive'), (Join-Path $env:LOCALAPPDATA 'Microsoft\Windows\WER\ReportQueue'), (Join-Path $env:ProgramData 'Microsoft\Windows\WER\ReportQueue')) |
  Where-Object { Test-Path $_ } | ForEach-Object { Get-ChildItem $_ -Directory -ErrorAction SilentlyContinue } |
  Where-Object { $_.LastWriteTime -ge $runStart -and $_.Name -match 'anime_compositor|msedgewebview2' })
$filesAfter = @(Files)
$new = @($filesAfter | Where-Object { $_ -notin $filesBefore -and (Split-Path -Leaf $_) -notlike "$Tag`_*" -and $_ -ne $windowFile })
# Named one by one, except a folder with more than five new files (the program's own disk cache
# of decoded drawings, B-161), which is counted with its size.
$newText = @($new | Group-Object { Split-Path -Parent $_ } | ForEach-Object {
  $d = $_.Name.Replace($root, '.').Replace($env:APPDATA, '%APPDATA%').Replace($env:LOCALAPPDATA, '%LOCALAPPDATA%')
  if ($_.Count -gt 5) { '{0} files ({1:N0} MiB) in `{2}`' -f $_.Count, (($_.Group | ForEach-Object { (Get-Item $_).Length } | Measure-Object -Sum).Sum / 1MB), $d }
  else { $_.Group | ForEach-Object { '`' + (Join-Path $d (Split-Path -Leaf $_)) + '`' } } }) -join ', '
foreach ($f in $new) { Remove-Item -Force $f }
Remove-Item -Recurse -Force $work
$recentNote = if ($null -eq $recentBytes) { if (Test-Path $recent) { Remove-Item -Force $recent }; 'there was none before; any written was removed' } else {
  $now = if (Test-Path $recent) { [IO.File]::ReadAllBytes($recent) } else { $null }
  $changed = ($null -eq $now) -or ([Convert]::ToBase64String($now) -ne [Convert]::ToBase64String($recentBytes))
  [IO.File]::WriteAllBytes($recent, $recentBytes)
  if ($changed) { 'the runs changed it; it was put back byte for byte' } else { 'unchanged' }
}
$windowNote = if ($null -eq $windowBytes) { if (Test-Path $windowFile) { Remove-Item -Force $windowFile; 'there was none before; the one the runs wrote was deleted, so the next start is a first launch' } else { 'there was none before, and none is left' } } else {
  [IO.File]::WriteAllBytes($windowFile, $windowBytes); 'put back byte for byte'
}
$md.Add('## 6. Crash reports and files left behind')
$md.Add('')
$md.Add("Application event log entries (Application Error, Application Hang, Windows Error Reporting) naming this program or its web view since the run began: $($events.Count)$(if ($events.Count) { ' - ' + (($events | ForEach-Object { "$($_.TimeCreated.ToString('HH:mm:ss')) $($_.ProviderName) $($_.Id)" }) -join '; ') }). Windows Error Reporting folders for them: $($wer.Count).")
$md.Add('')
$md.Add("Files the runs created, apart from the photographs (searched: the program's settings and local data folders apart from the web view's own profile, verification/, Fixtures/reference_shot/ and the repository's top folder), all deleted: $(if ($new.Count) { $newText } else { 'none' }). The copies of the projects and drawings under %TEMP%, and anything written beside them, were deleted. The recent-projects list: $recentNote. The remembered window (``window.txt``): $windowNote.")
$md.Add('')
$summary['No crash reports (event log, Windows Error Reporting)'] = if ($events.Count -or $wer.Count) { "FAIL ($($events.Count) events, $($wer.Count) reports)" } else { 'PASS (none)' }

# ---- the table ---------------------------------------------------------------------------------

$cpu = (Get-CimInstance Win32_Processor | Select-Object -First 1).Name.Trim()
$gpu = (Get-CimInstance Win32_VideoController | Where-Object { $_.Name -match 'NVIDIA' } | Select-Object -First 1)
$os = Get-CimInstance Win32_OperatingSystem
$commit = (git -C $root rev-parse --short HEAD 2>$null)
$built = (Get-Item $exe).LastWriteTime.ToString('yyyy-MM-dd HH:mm')
$head = New-Object Collections.Generic.List[string]
$head.Add("# $Tag - the application itself: start-up, dragging the window, stability")
$head.Add('')
$head.Add("Written by ``tools/app_stability.ps1`` ($(if ($Part -contains 'all') { 'every part' } else { 'parts ' + ($Part -join ', ') + ' only' })), $($runStart.ToString('yyyy-MM-dd HH:mm')) to $((Get-Date).ToString('HH:mm')).")
$head.Add('')
$head.Add("Machine: $cpu, $($gpu.Name) (driver $($gpu.DriverVersion)), $([math]::Round($os.TotalVisibleMemorySize / 1MB)) GB, $($os.Caption) $($os.Version). Main display 3840 by 2160 at 150%. Build: release ``anime_compositor_app.exe`` built $built, at commit $commit. Other programs running, as measured before each part: " + (($loads.Keys | ForEach-Object { "$_ - $($loads[$_])" }) -join '; ') + '.')
$head.Add('')
$head.Add('No target for start-up time or window responsiveness exists in the requirements (documents 03 and 08, T-06 and P-20 were checked), so every time below is a measurement only, with no pass or fail. The pass/fail lines are things that either happened or did not: a crash, a hang, a blank window, a window in the wrong place, a process left running, and memory growth against T-06''s bound.')
$head.Add('')
$head.Add('## Summary')
$head.Add('')
$head.Add('| Check | Result |')
$head.Add('|---|---|')
foreach ($k in $summary.Keys) { $head.Add("| $k | $($summary[$k]) |") }
$head.Add('')
$md.InsertRange(0, $head)
[IO.File]::WriteAllText((Join-Path $out "$Tag`_app_stability_table.md"), (($md -join "`n") + "`n"))
"wrote verification/$Tag`_app_stability_table.md"
$summary.Keys | ForEach-Object { "$_ : $($summary[$_])" }
