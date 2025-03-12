import { useState, useEffect } from "react"
import { Link } from "react-router-dom"
import { Button } from "../components/ui/button"
import { Card, CardContent } from "../components/ui/card"
import { Tabs, TabsList, TabsTrigger } from "../components/ui/tabs"
import { Badge } from "../components/ui/badge"
import { Skeleton } from "../components/ui/skeleton"
import { Play, Heart, Star, Film, Tv, Trash2, SlidersHorizontal, Search } from "lucide-react"
import { Input } from "../components/ui/input"

// Mock favorites data
const favorites = [
    {
        id: "1",
        title: "Interstellar",
        description: "A team of explorers travel through a wormhole in space in an attempt to ensure humanity's survival.",
        image: "/placeholder.svg?height=300&width=200",
        type: "movie",
        year: 2014,
        rating: 8.7,
        genres: ["Sci-Fi", "Adventure", "Drama"],
        duration: "2h 49m",
        dateAdded: "2024-01-15",
    },
    {
        id: "2",
        title: "Breaking Bad",
        description:
            "A high school chemistry teacher diagnosed with inoperable lung cancer turns to manufacturing and selling methamphetamine.",
        image: "/placeholder.svg?height=300&width=200",
        type: "tv",
        year: "2008-2013",
        seasons: 5,
        episodes: 62,
        rating: 9.5,
        genres: ["Crime", "Drama", "Thriller"],
        dateAdded: "2023-12-20",
    },
    {
        id: "9",
        title: "Soul",
        description:
            "A musician who has lost his passion for music is transported out of his body and must find his way back with the help of an infant soul learning about herself.",
        image: "/placeholder.svg?height=300&width=200",
        type: "movie",
        year: 2020,
        rating: 8.0,
        genres: ["Animation", "Adventure", "Comedy"],
        duration: "1h 40m",
        dateAdded: "2024-02-05",
    },
    {
        id: "4",
        title: "Game of Thrones",
        description: "Nine noble families fight for control over the lands of Westeros, while an ancient enemy returns.",
        image: "/placeholder.svg?height=300&width=200",
        type: "tv",
        year: "2011-2019",
        seasons: 8,
        episodes: 73,
        rating: 9.2,
        genres: ["Action", "Adventure", "Drama"],
        dateAdded: "2023-11-10",
    },
    {
        id: "11",
        title: "Spider-Man: Into the Spider-Verse",
        description:
            "Teen Miles Morales becomes the Spider-Man of his universe, and must join with five spider-powered individuals from other dimensions to stop a threat for all realities.",
        image: "/placeholder.svg?height=300&width=200",
        type: "movie",
        year: 2018,
        rating: 8.4,
        genres: ["Animation", "Action", "Adventure"],
        duration: "1h 57m",
        dateAdded: "2024-01-30",
    },
    {
        id: "10",
        title: "Arcane",
        description:
            "Set in utopian Piltover and the oppressed underground of Zaun, the story follows the origins of two iconic League champions-and the power that will tear them apart.",
        image: "/placeholder.svg?height=300&width=200",
        type: "tv",
        year: "2021-Present",
        seasons: 1,
        episodes: 9,
        rating: 9.0,
        genres: ["Animation", "Action", "Adventure"],
        dateAdded: "2023-12-05",
    },
]

export default function FavoritesPage() {
    const [isLoading, setIsLoading] = useState(true)
    const [activeTab, setActiveTab] = useState("all")
    const [searchQuery, setSearchQuery] = useState("")
    const [sortBy, setSortBy] = useState("recently-added")
    const [filteredFavorites, setFilteredFavorites] = useState(favorites)
    const [viewMode, setViewMode] = useState("grid")

    // Simulate loading state
    useEffect(() => {
        const timer = setTimeout(() => {
            setIsLoading(false)
        }, 1000)

        return () => clearTimeout(timer)
    }, [])

    // Filter and sort favorites based on active tab, search query, and sort option
    useEffect(() => {
        let result = [...favorites]

        // Filter by content type
        if (activeTab === "movies") {
            result = result.filter((item) => item.type === "movie")
        } else if (activeTab === "tv") {
            result = result.filter((item) => item.type === "tv")
        }

        // Filter by search query
        if (searchQuery) {
            const query = searchQuery.toLowerCase()
            result = result.filter(
                (item) =>
                    item.title.toLowerCase().includes(query) ||
                    item.description.toLowerCase().includes(query) ||
                    item.genres.some((genre) => genre.toLowerCase().includes(query)),
            )
        }

        // Sort favorites
        if (sortBy === "recently-added") {
            result.sort((a, b) => new Date(b.dateAdded).getTime() - new Date(a.dateAdded).getTime())
        } else if (sortBy === "title-asc") {
            result.sort((a, b) => a.title.localeCompare(b.title))
        } else if (sortBy === "title-desc") {
            result.sort((a, b) => b.title.localeCompare(a.title))
        } else if (sortBy === "rating") {
            result.sort((a, b) => b.rating - a.rating)
        }

        setFilteredFavorites(result)
    }, [activeTab, searchQuery, sortBy])

    // Remove from favorites
    const removeFromFavorites = (id: string) => {
        const updatedFavorites = favorites.filter((item) => item.id !== id)
        setFilteredFavorites(updatedFavorites)
    }

    if (isLoading) {
        return <FavoritesPageSkeleton />
    }

    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-3xl font-bold flex items-center">
                    <Heart className="mr-2 h-6 w-6 fill-red-500 text-red-500" /> Favorites
                </h1>
            </div>

            {/* Filters and Search */}
            <div className="flex flex-col md:flex-row gap-4 mb-6">
                <div className="relative flex-1">
                    <Search className="absolute left-3 top-1/2 -translate-y-1/2 h-4 w-4 text-muted-foreground" />
                    <Input
                        type="text"
                        placeholder="Search your favorites..."
                        className="pl-9"
                        value={searchQuery}
                        onChange={(e) => setSearchQuery(e.target.value)}
                    />
                </div>

                <div className="flex gap-2">
                    <Button
                        variant="outline"
                        className="gap-2"
                        onClick={() => {
                            setSortBy(sortBy === "recently-added" ? "rating" : "recently-added")
                        }}
                    >
                        <SlidersHorizontal className="h-4 w-4" />
                        <span className="hidden sm:inline">Sort by:</span>
                        {sortBy === "recently-added" ? "Recently Added" : "Rating"}
                    </Button>

                    <div className="flex border rounded-md overflow-hidden">
                        <Button
                            variant={viewMode === "grid" ? "default" : "ghost"}
                            size="sm"
                            className="rounded-none"
                            onClick={() => setViewMode("grid")}
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                strokeWidth="2"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                            >
                                <rect x="3" y="3" width="7" height="7" />
                                <rect x="14" y="3" width="7" height="7" />
                                <rect x="3" y="14" width="7" height="7" />
                                <rect x="14" y="14" width="7" height="7" />
                            </svg>
                        </Button>
                        <Button
                            variant={viewMode === "list" ? "default" : "ghost"}
                            size="sm"
                            className="rounded-none"
                            onClick={() => setViewMode("list")}
                        >
                            <svg
                                xmlns="http://www.w3.org/2000/svg"
                                width="16"
                                height="16"
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                                strokeWidth="2"
                                strokeLinecap="round"
                                strokeLinejoin="round"
                            >
                                <line x1="3" y1="6" x2="21" y2="6" />
                                <line x1="3" y1="12" x2="21" y2="12" />
                                <line x1="3" y1="18" x2="21" y2="18" />
                            </svg>
                        </Button>
                    </div>
                </div>
            </div>

            {/* Content Tabs */}
            <Tabs defaultValue="all" value={activeTab} onValueChange={setActiveTab} className="mb-6">
                <TabsList>
                    <TabsTrigger value="all">All Favorites</TabsTrigger>
                    <TabsTrigger value="movies">Movies</TabsTrigger>
                    <TabsTrigger value="tv">TV Shows</TabsTrigger>
                </TabsList>
            </Tabs>

            {/* Content Display */}
            {filteredFavorites.length === 0 ? (
                <div className="text-center py-12 animate-fade-in">
                    <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-muted mb-4">
                        <Heart className="h-8 w-8 text-muted-foreground" />
                    </div>
                    <h2 className="text-xl font-semibold mb-2">No favorites found</h2>
                    <p className="text-muted-foreground mb-6">
                        {searchQuery
                            ? "We couldn't find any favorites matching your search criteria."
                            : "You haven't added any favorites yet. Start exploring and add some!"}
                    </p>
                    {searchQuery && <Button onClick={() => setSearchQuery("")}>Clear Search</Button>}
                    {!searchQuery && (
                        <Button asChild>
                            <Link to="/discover">Discover Content</Link>
                        </Button>
                    )}
                </div>
            ) : viewMode === "grid" ? (
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6 animate-fade-in">
                    {filteredFavorites.map((item) => (
                        <FavoriteGridCard key={item.id} item={item} onRemove={() => removeFromFavorites(item.id)} />
                    ))}
                </div>
            ) : (
                <div className="space-y-4 animate-fade-in">
                    {filteredFavorites.map((item) => (
                        <FavoriteListCard key={item.id} item={item} onRemove={() => removeFromFavorites(item.id)} />
                    ))}
                </div>
            )}
        </div>
    )
}

// Favorite Grid Card Component
function FavoriteGridCard({ item, onRemove }: { item: any; onRemove: () => void }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <div className="relative aspect-[2/3]">
                <Link to={`/content/${item.id}`}>
                    <img
                        src={item.image || "/placeholder.svg"}
                        alt={item.title}
                        className="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                    />
                    <div className="absolute top-2 right-2">
                        {item.type === "movie" ? (
                            <Badge variant="secondary" className="bg-blue-500/20 text-blue-500">
                                <Film className="h-3 w-3 mr-1" /> Movie
                            </Badge>
                        ) : (
                            <Badge variant="secondary" className="bg-purple-500/20 text-purple-500">
                                <Tv className="h-3 w-3 mr-1" /> TV
                            </Badge>
                        )}
                    </div>
                    <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end">
                        <div className="p-4 w-full">
                            <Button size="sm" variant="secondary" className="w-full gap-2">
                                <Play className="h-4 w-4" /> Play
                            </Button>
                        </div>
                    </div>
                </Link>
                <Button
                    variant="destructive"
                    size="icon"
                    className="absolute top-2 left-2 h-8 w-8 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                    onClick={(e) => {
                        e.preventDefault()
                        onRemove()
                    }}
                >
                    <Trash2 className="h-4 w-4" />
                </Button>
            </div>
            <CardContent className="p-3">
                <div className="flex items-start justify-between">
                    <div>
                        <h3 className="font-medium line-clamp-1">{item.title}</h3>
                        <p className="text-sm text-muted-foreground">
                            {item.type === "movie" ? `${item.year} • ${item.duration}` : `${item.year} • ${item.seasons} Seasons`}
                        </p>
                    </div>
                    <Badge variant="outline" className="ml-2 bg-primary/10">
                        {item.rating}
                    </Badge>
                </div>
                <div className="mt-2 flex flex-wrap gap-1">
                    {item.genres.slice(0, 2).map((genre: string) => (
                        <Badge key={genre} variant="secondary" className="text-xs">
                            {genre}
                        </Badge>
                    ))}
                    {item.genres.length > 2 && (
                        <Badge variant="secondary" className="text-xs">
                            +{item.genres.length - 2}
                        </Badge>
                    )}
                </div>
            </CardContent>
        </Card>
    )
}

// Favorite List Card Component
function FavoriteListCard({ item, onRemove }: { item: any; onRemove: () => void }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <div className="flex flex-col sm:flex-row">
                <div className="relative w-full sm:w-48 aspect-video sm:aspect-[2/3]">
                    <Link to={`/content/${item.id}`}>
                        <img
                            src={item.image || "/placeholder.svg"}
                            alt={item.title}
                            className="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                        />
                        <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent sm:opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-center justify-center">
                            <Button size="sm" variant="secondary" className="gap-2">
                                <Play className="h-4 w-4" /> Play
                            </Button>
                        </div>
                    </Link>
                    <Button
                        variant="destructive"
                        size="icon"
                        className="absolute top-2 left-2 h-8 w-8 opacity-0 group-hover:opacity-100 transition-opacity duration-300"
                        onClick={onRemove}
                    >
                        <Trash2 className="h-4 w-4" />
                    </Button>
                </div>
                <div className="p-4 flex-1">
                    <div className="flex items-start justify-between mb-2">
                        <div>
                            <h3 className="font-medium text-lg">{item.title}</h3>
                            <div className="flex items-center gap-3 text-sm text-muted-foreground mt-1">
                                <span>{item.type === "movie" ? item.year : item.year}</span>
                                <span className="flex items-center">
                                    <Star className="h-3 w-3 mr-1 fill-yellow-400 text-yellow-400" />
                                    {item.rating}
                                </span>
                                {item.type === "movie" ? (
                                    <span>{item.duration}</span>
                                ) : (
                                    <span>
                                        {item.seasons} {item.seasons === 1 ? "Season" : "Seasons"}
                                    </span>
                                )}
                            </div>
                        </div>
                        {item.type === "movie" ? (
                            <Badge variant="secondary" className="bg-blue-500/20 text-blue-500">
                                <Film className="h-3 w-3 mr-1" /> Movie
                            </Badge>
                        ) : (
                            <Badge variant="secondary" className="bg-purple-500/20 text-purple-500">
                                <Tv className="h-3 w-3 mr-1" /> TV
                            </Badge>
                        )}
                    </div>
                    <p className="text-sm text-muted-foreground line-clamp-2 mb-3">{item.description}</p>
                    <div className="flex flex-wrap gap-1">
                        {item.genres.map((genre: string) => (
                            <Badge key={genre} variant="secondary" className="text-xs">
                                {genre}
                            </Badge>
                        ))}
                    </div>
                </div>
            </div>
        </Card>
    )
}

// Loading Skeleton
function FavoritesPageSkeleton() {
    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <div className="flex items-center justify-between mb-6">
                <Skeleton className="h-10 w-40" />
            </div>

            <div className="flex flex-col md:flex-row gap-4 mb-6">
                <Skeleton className="h-10 flex-1" />
                <div className="flex gap-2">
                    <Skeleton className="h-10 w-40" />
                    <Skeleton className="h-10 w-20" />
                </div>
            </div>

            <Skeleton className="h-10 w-96 mb-6" />

            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                {Array.from({ length: 10 }).map((_, i) => (
                    <div key={i} className="flex flex-col space-y-3">
                        <Skeleton className="aspect-[2/3] rounded-md" />
                        <Skeleton className="h-5 w-full" />
                        <Skeleton className="h-4 w-2/3" />
                        <Skeleton className="h-4 w-1/2" />
                    </div>
                ))}
            </div>
        </div>
    )
}


