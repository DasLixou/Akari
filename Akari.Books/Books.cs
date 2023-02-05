using Akari.Models;
using Akari.Books.Views;
using Avalonia.Controls;

namespace Akari.Books;

public class Books : ISidebarElement
{
    public uint Priority => 20;

    public string Title => "Books";

    public string OutlinedIcon => "Assets/Book.svg";

    public bool IsPrimary => true;

    public UserControl Content => new MainPage();
}