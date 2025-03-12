import { Button } from "../components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "../components/ui/card"
import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "../components/ui/dialog"
import { Input } from "../components/ui/input"
import { Label } from "../components/ui/label"
import { ListPlus, Plus } from "lucide-react"

export default function PlaylistsPage() {
    // Mock data for playlists
    const playlists = [
        {
            id: 1,
            name: "Weekend Watchlist",
            description: "Movies and shows to watch on the weekend",
            itemCount: 12,
        },
        {
            id: 2,
            name: "Sci-Fi Collection",
            description: "Best science fiction content",
            itemCount: 8,
        },
        {
            id: 3,
            name: "Documentaries",
            description: "Interesting documentaries to learn from",
            itemCount: 5,
        },
        {
            id: 4,
            name: "Comedy Night",
            description: "Funny movies and shows for a good laugh",
            itemCount: 10,
        },
    ]

    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-3xl font-bold">Your Playlists</h1>
                <Dialog>
                    <DialogTrigger asChild>
                        <Button>
                            <Plus className="mr-2 h-4 w-4" />
                            Create Playlist
                        </Button>
                    </DialogTrigger>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>Create New Playlist</DialogTitle>
                            <DialogDescription>Give your playlist a name and description to get started.</DialogDescription>
                        </DialogHeader>
                        <div className="grid gap-4 py-4">
                            <div className="grid gap-2">
                                <Label htmlFor="playlist-name">Playlist Name</Label>
                                <Input id="playlist-name" placeholder="My Awesome Playlist" />
                            </div>
                            <div className="grid gap-2">
                                <Label htmlFor="playlist-description">Description (Optional)</Label>
                                <Input id="playlist-description" placeholder="A collection of my favorite content" />
                            </div>
                        </div>
                        <DialogFooter>
                            <Button>Create Playlist</Button>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            </div>

            {playlists.length === 0 ? (
                <div className="flex flex-col items-center justify-center py-12">
                    <ListPlus className="h-16 w-16 text-muted-foreground mb-4" />
                    <h2 className="text-xl font-medium mb-2">No playlists yet</h2>
                    <p className="text-muted-foreground mb-6">Create your first playlist to organize your favorite content</p>
                    <Dialog>
                        <DialogTrigger asChild>
                            <Button>
                                <Plus className="mr-2 h-4 w-4" />
                                Create Playlist
                            </Button>
                        </DialogTrigger>
                        <DialogContent>
                            <DialogHeader>
                                <DialogTitle>Create New Playlist</DialogTitle>
                                <DialogDescription>Give your playlist a name and description to get started.</DialogDescription>
                            </DialogHeader>
                            <div className="grid gap-4 py-4">
                                <div className="grid gap-2">
                                    <Label htmlFor="playlist-name">Playlist Name</Label>
                                    <Input id="playlist-name" placeholder="My Awesome Playlist" />
                                </div>
                                <div className="grid gap-2">
                                    <Label htmlFor="playlist-description">Description (Optional)</Label>
                                    <Input id="playlist-description" placeholder="A collection of my favorite content" />
                                </div>
                            </div>
                            <DialogFooter>
                                <Button>Create Playlist</Button>
                            </DialogFooter>
                        </DialogContent>
                    </Dialog>
                </div>
            ) : (
                <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                    {playlists.map((playlist) => (
                        <Card key={playlist.id}>
                            <CardHeader>
                                <CardTitle>{playlist.name}</CardTitle>
                                <CardDescription>{playlist.description}</CardDescription>
                            </CardHeader>
                            <CardContent>
                                <p className="text-sm text-muted-foreground">
                                    {playlist.itemCount} {playlist.itemCount === 1 ? "item" : "items"}
                                </p>
                            </CardContent>
                            <CardFooter className="flex justify-between">
                                <Button variant="outline">View</Button>
                                <Button variant="ghost" size="icon">
                                    <Plus className="h-4 w-4" />
                                </Button>
                            </CardFooter>
                        </Card>
                    ))}
                    <Card className="flex flex-col items-center justify-center border-dashed">
                        <CardHeader>
                            <CardTitle className="text-center">Create New Playlist</CardTitle>
                        </CardHeader>
                        <CardContent>
                            <Dialog>
                                <DialogTrigger asChild>
                                    <Button variant="outline" size="icon" className="h-12 w-12 rounded-full">
                                        <Plus className="h-6 w-6" />
                                    </Button>
                                </DialogTrigger>
                                <DialogContent>
                                    <DialogHeader>
                                        <DialogTitle>Create New Playlist</DialogTitle>
                                        <DialogDescription>Give your playlist a name and description to get started.</DialogDescription>
                                    </DialogHeader>
                                    <div className="grid gap-4 py-4">
                                        <div className="grid gap-2">
                                            <Label htmlFor="playlist-name">Playlist Name</Label>
                                            <Input id="playlist-name" placeholder="My Awesome Playlist" />
                                        </div>
                                        <div className="grid gap-2">
                                            <Label htmlFor="playlist-description">Description (Optional)</Label>
                                            <Input id="playlist-description" placeholder="A collection of my favorite content" />
                                        </div>
                                    </div>
                                    <DialogFooter>
                                        <Button>Create Playlist</Button>
                                    </DialogFooter>
                                </DialogContent>
                            </Dialog>
                        </CardContent>
                    </Card>
                </div>
            )}
        </div>
    )
}
