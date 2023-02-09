using Avalonia;
using Avalonia.Controls;
using Avalonia.Media;

namespace Akari.Models;

public interface ISidebarElement
{
    uint Priority { get; }

    string Title { get; }
    string OutlinedIcon { get; }

    bool IsPrimary { get; }

    UserControl Content { get; }
}