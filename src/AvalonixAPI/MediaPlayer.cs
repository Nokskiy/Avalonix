using System;
using System.Threading;
using NAudio.Wave;

namespace Avalonix.AvalonixAPI;

public static class MediaPlayer
{
    private static Thread _playbackThread = null!;
    private static AudioFileReader _audioFile = null!;
    private static WasapiOut _output = null!;
    private static float Volume { get; set; } = 1;

    public static PlaybackState State => _output.PlaybackState;

    public static void Play(string path)
    {
        _playbackThread = new Thread(() =>
        {
            _audioFile = new AudioFileReader(path);
            _output = new WasapiOut();
            _output.Init(_audioFile);
            _output.Play();
            while (_output?.PlaybackState != PlaybackState.Stopped)
            {
                if (_output != null)
                    _output.Volume = Volume;
                Thread.Sleep(1000);
            }
        });
        _playbackThread.Start();
    }

    public static void Stop()
    {
        if (_output?.PlaybackState == PlaybackState.Playing)
        {
            _output.Stop();
            _output = null!;
        }
        _playbackThread = null!;
    }

    public static void Pause()
    {
        if (_output.PlaybackState == PlaybackState.Playing) _output.Pause();
    }

    public static void Continue()
    {
        if (_output.PlaybackState == PlaybackState.Paused) _output.Play();  
    } 

    public static void ChangeVolume(float volume) => Volume = volume;
    
    public static TimeSpan PlaybackTime => _audioFile.CurrentTime;

    public static TimeSpan TotalMusicTime => _audioFile.TotalTime;

}