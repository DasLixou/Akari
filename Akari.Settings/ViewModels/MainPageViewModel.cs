namespace Akari.Settings.ViewModels;

public class MainPageViewModel : ViewModelBase
{
    public IReadOnlyList<SettingsMenuButton> SettingsMenuButtons { get; set; } = new List<SettingsMenuButton>() {
        new() { Title = "WiFi", IconPath = "Assets/WiFi.svg" },
        new() { Title = "Services", IconPath = "Assets/TransparentCube.svg" },
        new() { Title = "SSH", IconPath = "Assets/FolderArrowDown.svg" },
        new() { Title = "Updates", IconPath = "Assets/ArrowPath.svg" },
        new() { Title = "Logs", IconPath = "Assets/Newspaper.svg" },
    };

    public struct SettingsMenuButton
    {
        public string Title { get; init; }
        public string IconPath { get; init; }
    }
}