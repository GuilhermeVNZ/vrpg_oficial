using System;
using System.Diagnostics;
using System.IO;

namespace VRPGServersLauncher
{
    class Program
    {
        static void Main(string[] args)
        {
            try
            {
                // Obter o diretório do executável
                string exeDir = Path.GetDirectoryName(System.Reflection.Assembly.GetExecutingAssembly().Location);
                string scriptPath = Path.Combine(exeDir, "servers.bat");
                
                // Se o script .bat não existir, tentar o PowerShell
                if (!File.Exists(scriptPath))
                {
                    scriptPath = Path.Combine(exeDir, "servers.ps1");
                }
                
                if (!File.Exists(scriptPath))
                {
                    Console.WriteLine("❌ Script não encontrado: servers.ps1 ou servers.bat");
                    Console.WriteLine("Certifique-se de que o arquivo está no mesmo diretório do executável.");
                    Console.WriteLine("\nPressione qualquer tecla para sair...");
                    Console.ReadKey();
                    return;
                }
                
                // Preparar argumentos
                string arguments = string.Join(" ", args);
                
                ProcessStartInfo startInfo;
                
                if (scriptPath.EndsWith(".ps1"))
                {
                    // Executar script PowerShell
                    startInfo = new ProcessStartInfo
                    {
                        FileName = "powershell.exe",
                        Arguments = $"-ExecutionPolicy Bypass -File \"{scriptPath}\" {arguments}",
                        UseShellExecute = false,
                        CreateNoWindow = false
                    };
                }
                else
                {
                    // Executar script .bat
                    startInfo = new ProcessStartInfo
                    {
                        FileName = scriptPath,
                        Arguments = arguments,
                        UseShellExecute = false,
                        CreateNoWindow = false
                    };
                }
                
                using (Process process = Process.Start(startInfo))
                {
                    process.WaitForExit();
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine($"❌ Erro ao executar o launcher: {ex.Message}");
                Console.WriteLine("\nPressione qualquer tecla para sair...");
                Console.ReadKey();
            }
        }
    }
}
