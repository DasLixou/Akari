using Akari.Models;

namespace Akari.Sidebar;

public class SidebarImpl : ISidebarService
{
    public void Initialize()
    {
        Console.WriteLine("Load Sidebar applications... une moment, s'il vous plaît...");
        foreach (var ele in Utils.Find<ISidebarElement>())
        {
            Console.WriteLine("Loading {0}...", ele.Title);
        }
    }
}