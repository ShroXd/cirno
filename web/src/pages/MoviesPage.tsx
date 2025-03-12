import { useState, useEffect } from "react"
import { Link } from "react-router-dom"
import { Button } from "../components/ui/button"
import { Card, CardContent } from "../components/ui/card"
import { Tabs, TabsList, TabsTrigger } from "../components/ui/tabs"
import { Badge } from "../components/ui/badge"
import { Skeleton } from "../components/ui/skeleton"
import { Play, Plus, Filter, Star, Clock, Calendar, ChevronRight, Film } from "lucide-react"

// Mock library content (replace with actual data fetching)
const libraryContent = [
    {
        id: "movie-1",
        type: "movie",
        title: "The Shawshank Redemption",
        year: 1994,
        duration: "2h 22m",
        rating: 9.3,
        genres: ["Drama"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-2",
        type: "movie",
        title: "The Godfather",
        year: 1972,
        duration: "2h 55m",
        rating: 9.2,
        genres: ["Crime", "Drama"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-3",
        type: "movie",
        title: "The Dark Knight",
        year: 2008,
        duration: "2h 32m",
        rating: 9.0,
        genres: ["Action", "Crime", "Drama"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-4",
        type: "movie",
        title: "Dune",
        year: 2021,
        duration: "2h 35m",
        rating: 7.9,
        genres: ["Sci-Fi", "Adventure", "Drama"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-5",
        type: "movie",
        title: "Spirited Away",
        year: 2001,
        duration: "2h 5m",
        rating: 8.6,
        genres: ["Animation", "Adventure", "Family"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-6",
        type: "movie",
        title: "Interstellar",
        year: 2014,
        duration: "2h 49m",
        rating: 8.6,
        genres: ["Sci-Fi", "Adventure"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-7",
        type: "movie",
        title: "Inception",
        year: 2010,
        duration: "2h 28m",
        rating: 8.8,
        genres: ["Action", "Sci-Fi", "Thriller"],
        image: "/placeholder.svg?height=300&width=200",
    },
    {
        id: "movie-8",
        type: "movie",
        title: "Forrest Gump",
        year: 1994,
        duration: "2h 22m",
        rating: 8.8,
        genres: ["Drama", "Romance"],
        image: "/placeholder.svg?height=300&width=200",
    },
]

// Filter movies from library content
const movies = libraryContent.filter((item) => item.type === "movie")

// Movie categories
const movieCategories = [
    {
        id: "action-adventure",
        title: "Action & Adventure",
        items: movies.filter((movie) => movie.genres.includes("Action") || movie.genres.includes("Adventure")),
    },
    {
        id: "sci-fi",
        title: "Science Fiction",
        items: movies.filter((movie) => movie.genres.includes("Sci-Fi")),
    },
    {
        id: "drama",
        title: "Drama",
        items: movies.filter((movie) => movie.genres.includes("Drama")),
    },
    {
        id: "animation",
        title: "Animation",
        items: movies.filter((movie) => movie.genres.includes("Animation")),
    },
]

// Featured movie
const featuredMovie = {
    id: "featured-movie",
    title: "Dune: Part Two",
    description:
        "Paul Atreides unites with Chani and the Fremen while seeking revenge against the conspirators who destroyed his family.",
    image: "/placeholder.svg?height=600&width=1200",
    year: 2024,
    rating: 8.6,
    duration: "2h 46m",
    genres: ["Sci-Fi", "Adventure", "Drama"],
    director: "Denis Villeneuve",
    cast: ["Timothée Chalamet", "Zendaya", "Rebecca Ferguson"],
}

export default function MoviesPage() {
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
        return <MoviesPageSkeleton />
    }

    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <div className="flex items-center justify-between mb-6">
                <h1 className="text-3xl font-bold">Movies</h1>
                <Button variant="outline" size="sm" className="gap-2">
                    <Filter className="h-4 w-4" /> Filter
                </Button>
            </div>

            {/* Featured Movie */}
            <div className="relative w-full h-[50vh] mb-12 rounded-xl overflow-hidden animate-fade-in">
                <img
                    src={featuredMovie.image || "/placeholder.svg"}
                    alt={featuredMovie.title}
                    className="absolute inset-0 w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent" />

                <div className="absolute bottom-0 left-0 right-0 p-6 md:p-8">
                    <div className="flex flex-col gap-4 max-w-3xl">
                        <Badge variant="secondary" className="w-fit bg-blue-500/20 text-blue-500">
                            <Film className="h-3 w-3 mr-1" /> Featured Movie
                        </Badge>

                        <h2 className="text-3xl md:text-5xl font-bold">{featuredMovie.title}</h2>
                        <p className="text-muted-foreground max-w-2xl">{featuredMovie.description}</p>

                        <div className="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
                            <div className="flex items-center">
                                <Star className="w-4 h-4 mr-1 fill-yellow-400 text-yellow-400" />
                                <span>{featuredMovie.rating}/10</span>
                            </div>
                            <div className="flex items-center">
                                <Calendar className="w-4 h-4 mr-1" />
                                <span>{featuredMovie.year}</span>
                            </div>
                            <div className="flex items-center">
                                <Clock className="w-4 h-4 mr-1" />
                                <span>{featuredMovie.duration}</span>
                            </div>
                        </div>

                        <div className="flex gap-3 mt-2">
                            <Button className="gap-2" asChild>
                                <Link to={`/content/${featuredMovie.id}`}>
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

            {/* Movie Categories */}
            <Tabs defaultValue="all" value={activeTab} onValueChange={setActiveTab} className="mb-6">
                <TabsList>
                    <TabsTrigger value="all">All Movies</TabsTrigger>
                    <TabsTrigger value="action">Action</TabsTrigger>
                    <TabsTrigger value="sci-fi">Sci-Fi</TabsTrigger>
                    <TabsTrigger value="drama">Drama</TabsTrigger>
                    <TabsTrigger value="animation">Animation</TabsTrigger>
                </TabsList>
            </Tabs>

            {activeTab === "all" ? (
                // Show all categories when "all" is selected
                <div className="space-y-12 animate-fade-in">
                    {movieCategories.map((category) => (
                        <section key={category.id}>
                            <div className="flex items-center justify-between mb-4">
                                <h2 className="text-2xl font-semibold">{category.title}</h2>
                                <Button variant="ghost" size="sm" className="gap-1" asChild>
                                    <Link to={`/view-all?category=movies&title=${category.title}`}>
                                        See All <ChevronRight className="h-4 w-4" />
                                    </Link>
                                </Button>
                            </div>

                            <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                                {category.items.slice(0, 5).map((movie) => (
                                    <MovieCard key={movie.id} movie={movie} />
                                ))}
                            </div>
                        </section>
                    ))}
                </div>
            ) : (
                // Show filtered movies when a specific category is selected
                <div className="animate-fade-in">
                    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                        {movies
                            .filter((movie) => {
                                if (activeTab === "action") return movie.genres.includes("Action") || movie.genres.includes("Adventure")
                                if (activeTab === "sci-fi") return movie.genres.includes("Sci-Fi")
                                if (activeTab === "drama") return movie.genres.includes("Drama")
                                if (activeTab === "animation") return movie.genres.includes("Animation")
                                return true
                            })
                            .map((movie) => (
                                <MovieCard key={movie.id} movie={movie} />
                            ))}
                    </div>
                </div>
            )}
        </div>
    )
}

// Movie Card Component
function MovieCard({ movie }: { movie: any }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <Link to={`/content/${movie.id}`}>
                <div className="relative aspect-[2/3]">
                    <img
                        src={movie.image || "/placeholder.svg"}
                        alt={movie.title}
                        className="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                    />
                    <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end">
                        <div className="p-4 w-full">
                            <Button size="sm" variant="secondary" className="w-full gap-2">
                                <Play className="h-4 w-4" /> Watch Now
                            </Button>
                        </div>
                    </div>
                </div>
                <CardContent className="p-3">
                    <div className="flex items-start justify-between">
                        <div>
                            <h3 className="font-medium line-clamp-1">{movie.title}</h3>
                            <p className="text-sm text-muted-foreground">
                                {movie.year} • {movie.duration}
                            </p>
                        </div>
                        <Badge variant="outline" className="ml-2 bg-primary/10">
                            {movie.rating}
                        </Badge>
                    </div>
                    <div className="mt-2 flex flex-wrap gap-1">
                        {movie.genres.slice(0, 2).map((genre: string) => (
                            <Badge key={genre} variant="secondary" className="text-xs">
                                {genre}
                            </Badge>
                        ))}
                        {movie.genres.length > 2 && (
                            <Badge variant="secondary" className="text-xs">
                                +{movie.genres.length - 2}
                            </Badge>
                        )}
                    </div>
                </CardContent>
            </Link>
        </Card>
    )
}

// Loading Skeleton
function MoviesPageSkeleton() {
    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <div className="flex items-center justify-between mb-6">
                <Skeleton className="h-10 w-40" />
                <Skeleton className="h-9 w-24" />
            </div>

            {/* Featured Movie Skeleton */}
            <Skeleton className="w-full h-[50vh] mb-12 rounded-xl" />

            {/* Tabs Skeleton */}
            <Skeleton className="h-10 w-96 mb-6" />

            {/* Categories Skeleton */}
            {[1, 2, 3, 4].map((category) => (
                <div key={category} className="mb-12">
                    <div className="flex items-center justify-between mb-4">
                        <Skeleton className="h-8 w-48" />
                        <Skeleton className="h-8 w-24" />
                    </div>
                    <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-6">
                        {[1, 2, 3, 4, 5].map((i) => (
                            <div key={i} className="flex flex-col space-y-3">
                                <Skeleton className="aspect-[2/3] rounded-md" />
                                <Skeleton className="h-5 w-full" />
                                <Skeleton className="h-4 w-2/3" />
                                <Skeleton className="h-4 w-1/2" />
                            </div>
                        ))}
                    </div>
                </div>
            ))}
        </div>
    )
}

