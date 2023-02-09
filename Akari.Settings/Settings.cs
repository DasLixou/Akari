using Akari.Models;
using Akari.Settings.Views;
using Avalonia.Controls;

namespace Akari.Settings;

public class Settings : ISidebarElement
{
    public uint Priority => 100;

    public string Title => "Settings";

    public string OutlinedIcon => "Assets/Cog8.svg";

    public bool IsPrimary => true; // TODO: Change this back when it works

    public UserControl Content => new MainPage();
}