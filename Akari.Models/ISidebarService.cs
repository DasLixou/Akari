namespace Akari.Models;

public interface ISidebarService
{
    IReadOnlyList<ISidebarElement> PrimaryElements { get; }
    IReadOnlyList<ISidebarElement> SecondaryElements { get; }

    void Initialize();
}