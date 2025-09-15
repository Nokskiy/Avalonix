using Avalonix.Models.UserSettings.AvalonixSettingsFiles;

namespace Avalonix.Models.UserSettings;

public interface ISettings
{
    Theme.Theme Theme { get; set; }
    AvalonixSettings Avalonix { get; set; }
}