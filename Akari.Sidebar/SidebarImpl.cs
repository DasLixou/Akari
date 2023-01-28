using Akari.Models;
using AuroraModularis.Core;

namespace Akari.Sidebar;

public class SidebarImpl : ISidebarService
{
    public IReadOnlyList<ISidebarElement> Elements { get; set; }

    public void Initialize()
    {
        var elements = new List<ISidebarElement>();
        Console.WriteLine("Load Sidebar applications... une moment, s'il vous plaît...");
        foreach (var ele in Container.Current.Resolve<ITypeFinder>().FindAndResolveTypes<ISidebarElement>().OrderBy(e => e.Priority))
        {
            Console.WriteLine("Loading {0}...", ele.Title);
            elements.Add(ele);
        }
        Elements = elements;
    }
}