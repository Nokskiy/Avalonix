﻿using Avalonia;
using System;
using NeoSimpleLogger;
using AvalonixAPI;
using System.Collections.Generic;
using Avalonix.AvalonixAPI;
using System.Threading;

namespace Avalonix;


public static class Program
{
        public static readonly Logger Logger = new(Logger.TypeLogger.Console);

        [STAThread]
        public static void Main(string[] args)
        {
                Logger.Info(".avalonix path - " + DiskManager.SettingsPath);
                /*
                          Thread testPlaylistThread = new Thread(() =>
                                {
                                        List<SongData> songs = [new ("blood // water", "test.mp3")];

                                        string playlistName = "TestPlaylist";
                                        PlaylistsManager.CreateNewPlaylist(new PlaylistData(playlistName, songs));

                                        PlaylistsManager.PlayPlaylist(playlistName);

                                });
                testPlaylistThread.Start();      
                */
                BuildAvaloniaApp().StartWithClassicDesktopLifetime(args);
        }

        private static AppBuilder BuildAvaloniaApp()
        {

                Logger.Info("Building App");
#if DEBUG
                Logger.CallStack = true;
#endif
                return AppBuilder.Configure<App>()
                    .UsePlatformDetect()
                    .WithInterFont()
                    .LogToTrace();

        }
}