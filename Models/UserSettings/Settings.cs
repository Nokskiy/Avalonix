using System.Text.Json.Serialization;
using Avalonix.Models.UserSettings.AvalonixSettingsFiles;

namespace Avalonix.Models.UserSettings;

public class Settings : ISettings
{
    [JsonIgnore] public Theme.Theme Theme { get; set; }
    public AvalonixSettings Avalonix { get; set; }
}