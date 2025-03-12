import { useState, useEffect } from "react"
import { Link } from "react-router-dom"
import { Button } from "../components/ui/button"
import { Card, CardContent } from "../components/ui/card"
import { Tabs, TabsList, TabsTrigger } from "../components/ui/tabs"
import { Badge } from "../components/ui/badge"
import { Skeleton } from "../components/ui/skeleton"
import { Play, Plus, TrendingUp, Award, Calendar, Star, Clock, Filter, ChevronRight } from "lucide-react"
import { cn } from "../lib/utils"

// Mock data for featured content
const featuredContent = {
    id: "featured-1",
    title: "The Last of Us",
    description:
        "After a global pandemic destroys civilization, a hardened survivor takes charge of a 14-year-old girl who may be humanity's last hope.",
    image: "/placeholder.svg?height=600&width=1200",
    type: "tv",
    year: 2023,
    rating: 8.8,
    genres: ["Drama", "Action", "Horror"],
}

// Mock data for trending content
const trendingContent = [
    {
        id: "1",
        title: "Dune: Part Two",
        image: "/placeholder.svg?height=400&width=600",
        type: "movie",
        year: 2024,
        rating: 8.6,
        genres: ["Sci-Fi", "Adventure"],
    },
    {
        id: "2",
        title: "House of the Dragon",
        image: "/placeholder.svg?height=400&width=600",
        type: "tv",
        year: 2022,
        rating: 8.5,
        genres: ["Fantasy", "Drama"],
    },
    {
        id: "3",
        title: "Oppenheimer",
        image: "/placeholder.svg?height=400&width=600",
        type: "movie",
        year: 2023,
        rating: 8.9,
        genres: ["Biography", "Drama"],
    },
    {
        id: "4",
        title: "The Bear",
        image: "/placeholder.svg?height=400&width=600",
        type: "tv",
        year: 2022,
        rating: 8.7,
        genres: ["Comedy", "Drama"],
    },
    {
        id: "5",
        title: "Poor Things",
        image: "/placeholder.svg?height=400&width=600",
        type: "movie",
        year: 2023,
        rating: 8.4,
        genres: ["Comedy", "Romance", "Sci-Fi"],
    },
]

// Mock data for genres
const genres = [
    { id: "action", name: "Action", count: 245, color: "bg-red-500/10 text-red-500" },
    { id: "comedy", name: "Comedy", count: 189, color: "bg-yellow-500/10 text-yellow-500" },
    { id: "drama", name: "Drama", count: 327, color: "bg-blue-500/10 text-blue-500" },
    { id: "sci-fi", name: "Sci-Fi", count: 156, color: "bg-purple-500/10 text-purple-500" },
    { id: "horror", name: "Horror", count: 112, color: "bg-green-500/10 text-green-500" },
    { id: "romance", name: "Romance", count: 98, color: "bg-pink-500/10 text-pink-500" },
    { id: "thriller", name: "Thriller", count: 203, color: "bg-orange-500/10 text-orange-500" },
    { id: "fantasy", name: "Fantasy", count: 134, color: "bg-indigo-500/10 text-indigo-500" },
]

// Mock data for curated collections
const curatedCollections = [
    {
        id: "award-winners",
        title: "Award Winners",
        icon: <Award className="h-5 w-5" />,
        items: [
            {
                id: "aw1",
                title: "Everything Everywhere All at Once",
                image: "/placeholder.svg?height=300&width=200",
                type: "movie",
                year: 2022,
                award: "Academy Award for Best Picture",
            },
            {
                id: "aw2",
                title: "Succession",
                image: "/placeholder.svg?height=300&width=200",
                type: "tv",
                year: 2023,
                award: "Emmy for Outstanding Drama Series",
            },
            {
                id: "aw3",
                title: "Parasite",
                image: "/placeholder.svg?height=300&width=200",
                type: "movie",
                year: 2019,
                award: "Academy Award for Best Picture",
            },
            {
                id: "aw4",
                title: "The Crown",
                image: "/placeholder.svg?height=300&width=200",
                type: "tv",
                year: 2021,
                award: "Golden Globe for Best TV Series",
            },
        ],
    },
    {
        id: "new-releases",
        title: "New Releases",
        icon: <Calendar className="h-5 w-5" />,
        items: [
            {
                id: "nr1",
                title: "The Penguin",
                image: "/placeholder.svg?height=300&width=200",
                type: "tv",
                year: 2024,
                releaseDate: "March 2024",
            },
            {
                id: "nr2",
                title: "Challengers",
                image: "/placeholder.svg?height=300&width=200",
                type: "movie",
                year: 2024,
                releaseDate: "April 2024",
            },
            {
                id: "nr3",
                title: "Fallout",
                image: "/placeholder.svg?height=300&width=200",
                type: "tv",
                year: 2024,
                releaseDate: "April 2024",
            },
            {
                id: "nr4",
                title: "Civil War",
                image: "/placeholder.svg?height=300&width=200",
                type: "movie",
                year: 2024,
                releaseDate: "April 2024",
            },
        ],
    },
    {
        id: "top-rated",
        title: "Top Rated",
        icon: <Star className="h-5 w-5" />,
        items: [
            {
                id: "tr1",
                title: "Breaking Bad",
                image: "/placeholder.svg?height=300&width=200",
                type: "tv",
                year: "2008-2013",
                rating: 9.5,
            },
            {
                id: "tr2",
                title: "The Shawshank Redemption",
                image: "/placeholder.svg?height=300&width=200",
                type: "movie",
                year: 1994,
                rating: 9.3,
            },
            {
                id: "tr3",
                title: "The Wire",
                image: "/placeholder.svg?height=300&width=200",
                type: "tv",
                year: "2002-2008",
                rating: 9.3,
            },
            {
                id: "tr4",
                title: "The Godfather",
                image: "/placeholder.svg?height=300&width=200",
                type: "movie",
                year: 1972,
                rating: 9.2,
            },
        ],
    },
]

// Mock data for continue watching
const continueWatching = [
    {
        id: "cw1",
        title: "Severance",
        image: "/placeholder.svg?height=300&width=500",
        type: "tv",
        progress: 65,
        episode: "S01E07",
        remainingTime: "22 min left",
    },
    {
        id: "cw2",
        title: "The Batman",
        image: "/placeholder.svg?height=300&width=500",
        type: "movie",
        progress: 40,
        remainingTime: "1h 35min left",
    },
    {
        id: "cw3",
        title: "Succession",
        image: "/placeholder.svg?height=300&width=500",
        type: "tv",
        progress: 85,
        episode: "S03E09",
        remainingTime: "8 min left",
    },
]

export default function DiscoverPage() {
    const [isLoading, setIsLoading] = useState(true)
    const [activeTab, setActiveTab] = useState("all")

    // Simulate loading state
    useEffect(() => {
        const timer = setTimeout(() => {
            setIsLoading(false)
        }, 1000)

        return () => clearTimeout(timer)
    }, [])

    if (isLoading) {
        return <DiscoverPageSkeleton />
    }

    return (
        <div className="container mx-auto py-6 px-4 md:px-6 overflow-y-auto h-screen">
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-3xl font-bold">Discover</h1>
                <Button variant="outline" size="sm" className="gap-2">
                    <Filter className="h-4 w-4" /> Filters
                </Button>
            </div>

            {/* Featured Content Hero */}
            <div className="relative w-full h-[50vh] mb-12 rounded-xl overflow-hidden animate-fade-in">
                <img
                    src={featuredContent.image || "/placeholder.svg"}
                    alt={featuredContent.title}
                    className="absolute inset-0 w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent" />

                <div className="absolute bottom-0 left-0 right-0 p-6 md:p-8">
                    <div className="flex flex-col gap-4 max-w-3xl">
                        <div className="flex flex-wrap gap-2">
                            {featuredContent.genres.map((genre) => (
                                <Badge key={genre} variant="secondary">
                                    {genre}
                                </Badge>
                            ))}
                        </div>

                        <h2 className="text-3xl md:text-5xl font-bold">{featuredContent.title}</h2>
                        <p className="text-muted-foreground max-w-2xl">{featuredContent.description}</p>

                        <div className="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
                            <div className="flex items-center">
                                <Star className="w-4 h-4 mr-1 fill-yellow-400 text-yellow-400" />
                                <span>{featuredContent.rating}/10</span>
                            </div>
                            <div className="flex items-center">
                                <Calendar className="w-4 h-4 mr-1" />
                                <span>{featuredContent.year}</span>
                            </div>
                        </div>

                        <div className="flex gap-3 mt-2">
                            <Button className="gap-2" asChild>
                                <Link to={`/content/${featuredContent.id}`}>
                                    <Play className="w-4 h-4" /> Watch Now
                                </Link>
                            </Button>
                            <Button variant="outline" className="gap-2">
                                <Plus className="w-4 h-4" /> Add to Playlist
                            </Button>
                        </div>
                    </div>
                </div>
            </div>

            {/* Continue Watching */}
            {continueWatching.length > 0 && (
                <section className="mb-12 animate-fade-in">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-2xl font-semibold flex items-center">
                            <Clock className="mr-2 h-5 w-5" /> Continue Watching
                        </h2>
                        <Button variant="ghost" size="sm" className="gap-1">
                            See All <ChevronRight className="h-4 w-4" />
                        </Button>
                    </div>

                    <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                        {continueWatching.map((item) => (
                            <ContinueWatchingCard key={item.id} item={item} />
                        ))}
                    </div>
                </section>
            )}

            {/* Trending Now */}
            <section className="mb-12 animate-fade-in">
                <div className="flex items-center justify-between mb-4">
                    <h2 className="text-2xl font-semibold flex items-center">
                        <TrendingUp className="mr-2 h-5 w-5" /> Trending Now
                    </h2>
                    <Button variant="ghost" size="sm" className="gap-1">
                        See All <ChevronRight className="h-4 w-4" />
                    </Button>
                </div>

                <Tabs defaultValue="all" value={activeTab} onValueChange={setActiveTab} className="mb-4">
                    <TabsList>
                        <TabsTrigger value="all">All</TabsTrigger>
                        <TabsTrigger value="movies">Movies</TabsTrigger>
                        <TabsTrigger value="tv">TV Shows</TabsTrigger>
                    </TabsList>
                </Tabs>

                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-6">
                    {trendingContent
                        .filter((item) =>
                            activeTab === "all" ? true : activeTab === "movies" ? item.type === "movie" : item.type === "tv",
                        )
                        .map((item) => (
                            <TrendingCard key={item.id} item={item} />
                        ))}
                </div>
            </section>

            {/* Browse by Genre */}
            <section className="mb-12 animate-fade-in">
                <h2 className="text-2xl font-semibold mb-6">Browse by Genre</h2>
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                    {genres.map((genre) => (
                        <GenreCard key={genre.id} genre={genre} />
                    ))}
                </div>
            </section>

            {/* Curated Collections */}
            {curatedCollections.map((collection) => (
                <section key={collection.id} className="mb-12 animate-fade-in">
                    <div className="flex items-center justify-between mb-4">
                        <h2 className="text-2xl font-semibold flex items-center">
                            {collection.icon}
                            <span className="ml-2">{collection.title}</span>
                        </h2>
                        <Button variant="ghost" size="sm" className="gap-1">
                            See All <ChevronRight className="h-4 w-4" />
                        </Button>
                    </div>

                    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-6">
                        {collection.items.map((item) => (
                            <CollectionCard key={item.id} item={item} collectionType={collection.id} />
                        ))}
                    </div>
                </section>
            ))}
        </div>
    )
}

// Trending Card Component
function TrendingCard({ item }: { item: any }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <Link to={`/content/${item.id}`}>
                <div className="relative aspect-[2/3]">
                    <img
                        src={item.image || "/placeholder.svg"}
                        alt={item.title}
                        className="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                    />
                    <div className="absolute top-2 left-2 flex items-center gap-1 bg-background/80 text-foreground px-2 py-1 rounded-md text-xs font-medium">
                        <TrendingUp className="h-3 w-3" /> Trending
                    </div>
                    <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end">
                        <div className="p-4 w-full">
                            <Button size="sm" variant="secondary" className="w-full gap-2">
                                <Play className="h-4 w-4" /> Play
                            </Button>
                        </div>
                    </div>
                </div>
                <CardContent className="p-3">
                    <div className="flex items-start justify-between">
                        <div>
                            <h3 className="font-medium line-clamp-1">{item.title}</h3>
                            <p className="text-sm text-muted-foreground">{item.year}</p>
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
                    </div>
                </CardContent>
            </Link>
        </Card>
    )
}

// Genre Card Component
function GenreCard({ genre }: { genre: any }) {
    return (
        <Link to={`/genre/${genre.id}`}>
            <Card
                className={cn(
                    "h-24 flex items-center justify-center text-center transition-all duration-300 hover:shadow-md overflow-hidden group",
                    genre.color,
                )}
            >
                <CardContent className="p-4 w-full">
                    <h3 className="font-semibold text-lg mb-1">{genre.name}</h3>
                    <p className="text-xs opacity-80">{genre.count} titles</p>
                </CardContent>
            </Card>
        </Link>
    )
}

// Collection Card Component
function CollectionCard({ item, collectionType }: { item: any; collectionType: string }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <Link to={`/content/${item.id}`}>
                <div className="relative aspect-[2/3]">
                    <img
                        src={item.image || "/placeholder.svg"}
                        alt={item.title}
                        className="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end">
                        <div className="p-4 w-full">
                            <Button size="sm" variant="secondary" className="w-full gap-2">
                                <Play className="h-4 w-4" /> Play
                            </Button>
                        </div>
                    </div>
                </div>
                <CardContent className="p-3">
                    <h3 className="font-medium line-clamp-1">{item.title}</h3>
                    <div className="flex items-center justify-between mt-1">
                        <p className="text-xs text-muted-foreground">{item.type === "movie" ? "Movie" : "TV Series"}</p>
                        {collectionType === "award-winners" && (
                            <Badge variant="outline" className="text-xs">
                                <Award className="h-3 w-3 mr-1" /> Award Winner
                            </Badge>
                        )}
                        {collectionType === "top-rated" && (
                            <Badge variant="outline" className="text-xs">
                                <Star className="h-3 w-3 mr-1 fill-yellow-400 text-yellow-400" /> {item.rating}
                            </Badge>
                        )}
                        {collectionType === "new-releases" && (
                            <Badge variant="outline" className="text-xs">
                                <Calendar className="h-3 w-3 mr-1" /> New
                            </Badge>
                        )}
                    </div>
                </CardContent>
            </Link>
        </Card>
    )
}

// Continue Watching Card Component
function ContinueWatchingCard({ item }: { item: any }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <Link to={`/watch/${item.id}`}>
                <div className="relative aspect-video">
                    <img
                        src={item.image || "/placeholder.svg"}
                        alt={item.title}
                        className="absolute inset-0 w-full h-full object-cover"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent flex flex-col justify-end">
                        <div className="p-4">
                            <h3 className="font-medium text-white">{item.title}</h3>
                            <div className="flex items-center justify-between text-xs text-white/80 mb-2">
                                <span>{item.type === "tv" ? item.episode : "Movie"}</span>
                                <span>{item.remainingTime}</span>
                            </div>
                            <div className="relative h-1 bg-white/30 rounded-full overflow-hidden">
                                <div
                                    className="absolute top-0 left-0 h-full bg-primary rounded-full"
                                    style={{ width: `${item.progress}%` }}
                                />
                            </div>
                        </div>
                    </div>
                    <div className="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300 bg-black/40">
                        <Button size="sm" variant="secondary" className="gap-2">
                            <Play className="h-4 w-4" /> Resume
                        </Button>
                    </div>
                </div>
            </Link>
        </Card>
    )
}

// Loading Skeleton
function DiscoverPageSkeleton() {
    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <div className="flex items-center justify-between mb-6">
                <Skeleton className="h-10 w-40" />
                <Skeleton className="h-9 w-24" />
            </div>

            {/* Featured Content Skeleton */}
            <Skeleton className="w-full h-[50vh] mb-12 rounded-xl" />

            {/* Continue Watching Skeleton */}
            <div className="mb-12">
                <div className="flex items-center justify-between mb-4">
                    <Skeleton className="h-8 w-48" />
                    <Skeleton className="h-8 w-24" />
                </div>
                <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
                    {[1, 2, 3].map((i) => (
                        <Skeleton key={i} className="aspect-video rounded-lg" />
                    ))}
                </div>
            </div>

            {/* Trending Skeleton */}
            <div className="mb-12">
                <div className="flex items-center justify-between mb-4">
                    <Skeleton className="h-8 w-48" />
                    <Skeleton className="h-8 w-24" />
                </div>
                <Skeleton className="h-10 w-64 mb-4" />
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-5 gap-6">
                    {[1, 2, 3, 4, 5].map((i) => (
                        <div key={i} className="flex flex-col">
                            <Skeleton className="aspect-[2/3] rounded-lg mb-2" />
                            <Skeleton className="h-5 w-full mb-2" />
                            <Skeleton className="h-4 w-2/3" />
                        </div>
                    ))}
                </div>
            </div>

            {/* Genres Skeleton */}
            <div className="mb-12">
                <Skeleton className="h-8 w-48 mb-6" />
                <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                    {[1, 2, 3, 4, 5, 6, 7, 8].map((i) => (
                        <Skeleton key={i} className="h-24 rounded-lg" />
                    ))}
                </div>
            </div>

            {/* Collections Skeleton */}
            {[1, 2, 3].map((collection) => (
                <div key={collection} className="mb-12">
                    <div className="flex items-center justify-between mb-4">
                        <Skeleton className="h-8 w-48" />
                        <Skeleton className="h-8 w-24" />
                    </div>
                    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-6">
                        {[1, 2, 3, 4].map((i) => (
                            <div key={i} className="flex flex-col">
                                <Skeleton className="aspect-[2/3] rounded-lg mb-2" />
                                <Skeleton className="h-5 w-full mb-2" />
                                <Skeleton className="h-4 w-2/3" />
                            </div>
                        ))}
                    </div>
                </div>
            ))}
        </div>
    )
}

