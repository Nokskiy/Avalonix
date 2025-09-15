using System.Threading.Tasks;
using Avalonix.Models.Media.TrackFiles;

namespace Avalonix.Models.Media.PlaylistFiles;

public interface IPlaylist
{ 
    Task AddTrack(Track track);
    Playlist MergePlaylist(Playlist[] otherPlaylists);
    Task RemoveTrack(Track track);
    Task RemoveDuplicativeTracks();
    Task SortTracks(SortPlaylistTrackFlags flags);
    Task Save();
    Task Play(int startSong = 0);
    void Stop();
    void Pause();
    void Resume();
}