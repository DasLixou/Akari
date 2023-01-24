using Akari.ViewModels;
using System;
using System.Windows.Input;

namespace Akari.Commands;

public class SwitchPageCommand : ICommand
{
    private MainWindowViewModel viewModel;

    public SwitchPageCommand(MainWindowViewModel viewModel)
    {
        this.viewModel = viewModel;
    }

    public event EventHandler CanExecuteChanged;

    public bool CanExecute(object parameter) => true;

    public void Execute(object parameter) => viewModel.SelectedIndex = int.Parse((string)parameter);
}