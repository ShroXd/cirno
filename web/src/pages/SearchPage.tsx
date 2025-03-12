import type React from "react"

import { useState, useEffect, useRef } from "react"
import { Button } from "../components/ui/button"
import { Input } from "../components/ui/input"
import { Card, CardContent } from "../components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "../components/ui/tabs"
import { Badge } from "../components/ui/badge"
import { Skeleton } from "../components/ui/skeleton"
import { Link } from "react-router-dom"
import { Play, SearchIcon, X, Film, Tv, Clock, TrendingUpIcon as Trending, Star } from "lucide-react"
import { cn } from "../lib/utils"

// Mock search results data
const mockSearchResults = {
    movies: [
        {
            id: "1",
            title: "Interstellar",
            year: 2014,
            type: "movie",
            image: "/placeholder.svg?height=300&width=200",
            rating: 8.7,
            genres: ["Sci-Fi", "Adventure", "Drama"],
        },
        {
            id: "3",
            title: "The Shawshank Redemption",
            year: 1994,
            type: "movie",
            image: "/placeholder.svg?height=300&width=200",
            rating: 9.3,
            genres: ["Drama"],
        },
        {
            id: "5",
            title: "Dune",
            year: 2021,
            type: "movie",
            image: "/placeholder.svg?height=300&width=200",
            rating: 8.0,
            genres: ["Sci-Fi", "Adventure"],
        },
        {
            id: "7",
            title: "The Batman",
            year: 2022,
            type: "movie",
            image: "/placeholder.svg?height=300&width=200",
            rating: 7.8,
            genres: ["Action", "Crime", "Drama"],
        },
    ],
    tvShows: [
        {
            id: "2",
            title: "Breaking Bad",
            years: "2008-2013",
            type: "tv",
            image: "/placeholder.svg?height=300&width=200",
            rating: 9.5,
            genres: ["Crime", "Drama", "Thriller"],
        },
        {
            id: "4",
            title: "Game of Thrones",
            years: "2011-2019",
            type: "tv",
            image: "/placeholder.svg?height=300&width=200",
            rating: 9.2,
            genres: ["Action", "Adventure", "Drama"],
        },
        {
            id: "6",
            title: "Succession",
            years: "2018-2023",
            type: "tv",
            image: "/placeholder.svg?height=300&width=200",
            rating: 8.8,
            genres: ["Drama"],
        },
        {
            id: "8",
            title: "Stranger Things",
            years: "2016-Present",
            type: "tv",
            image: "/placeholder.svg?height=300&width=200",
            rating: 8.7,
            genres: ["Drama", "Fantasy", "Horror"],
        },
    ],
}

// Popular searches
const popularSearches = ["Action movies", "Sci-Fi series", "New releases", "Award winners", "Comedy", "Documentaries"]

export default function SearchPage() {
    const [searchQuery, setSearchQuery] = useState("")
    const [isSearching, setIsSearching] = useState(false)
    const [searchResults, setSearchResults] = useState<any>(null)
    const [recentSearches, setRecentSearches] = useState<string[]>([])
    const [activeTab, setActiveTab] = useState("all")
    const searchInputRef = useRef<HTMLInputElement>(null)

    // Focus the search input on page load
    useEffect(() => {
        if (searchInputRef.current) {
            searchInputRef.current.focus()
        }

        // Load recent searches from localStorage
        const savedSearches = localStorage.getItem("recentSearches")
        if (savedSearches) {
            setRecentSearches(JSON.parse(savedSearches))
        }
    }, [])

    // Handle search
    const handleSearch = (query: string = searchQuery) => {
        if (!query.trim()) {
            setSearchResults(null)
            return
        }

        setIsSearching(true)

        // Simulate API call with timeout
        setTimeout(() => {
            setSearchResults(mockSearchResults)
            setIsSearching(false)

            // Save to recent searches
            if (!recentSearches.includes(query) && query.trim()) {
                const updatedSearches = [query, ...recentSearches.slice(0, 4)]
                setRecentSearches(updatedSearches)
                localStorage.setItem("recentSearches", JSON.stringify(updatedSearches))
            }
        }, 800)
    }

    // Clear search
    const clearSearch = () => {
        setSearchQuery("")
        setSearchResults(null)
        if (searchInputRef.current) {
            searchInputRef.current.focus()
        }
    }

    // Handle popular search click
    const handlePopularSearch = (query: string) => {
        setSearchQuery(query)
        handleSearch(query)
    }

    // Handle recent search click
    const handleRecentSearch = (query: string) => {
        setSearchQuery(query)
        handleSearch(query)
    }

    // Clear recent searches
    const clearRecentSearches = () => {
        setRecentSearches([])
        localStorage.removeItem("recentSearches")
    }

    // Get filtered results based on active tab
    const getFilteredResults = () => {
        if (!searchResults) return []

        if (activeTab === "all") {
            return [...searchResults.movies, ...searchResults.tvShows]
        } else if (activeTab === "movies") {
            return searchResults.movies
        } else {
            return searchResults.tvShows
        }
    }

    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            {/* Search Header */}
            <div className="mb-8">
                <h1 className="text-3xl font-bold mb-6">Search</h1>

                {/* Search Input */}
                <div className="relative max-w-3xl mx-auto">
                    <div className="relative flex items-center">
                        <SearchIcon className="absolute left-3 top-1/2 -translate-y-1/2 h-5 w-5 text-muted-foreground" />
                        <Input
                            ref={searchInputRef}
                            type="text"
                            placeholder="Search for movies, TV shows, actors..."
                            className={cn(
                                "pl-10 pr-10 py-6 text-lg transition-all duration-300 border-2",
                                searchQuery ? "border-primary" : "border-input",
                                "focus-visible:ring-2 focus-visible:ring-offset-0",
                            )}
                            value={searchQuery}
                            onChange={(e) => setSearchQuery(e.target.value)}
                            onKeyDown={(e) => e.key === "Enter" && handleSearch()}
                        />
                        {searchQuery && (
                            <button
                                className="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                                onClick={clearSearch}
                            >
                                <X className="h-5 w-5" />
                            </button>
                        )}
                    </div>

                    <Button className="absolute right-0 top-0 h-full rounded-l-none" onClick={() => handleSearch()}>
                        Search
                    </Button>
                </div>
            </div>

            {/* Search Results */}
            {isSearching ? (
                <div className="mt-12 animate-fade-in">
                    <h2 className="text-xl font-semibold mb-6">Searching...</h2>
                    <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                        {Array.from({ length: 8 }).map((_, i) => (
                            <div key={i} className="flex flex-col space-y-3">
                                <Skeleton className="h-[200px] w-full rounded-md" />
                                <Skeleton className="h-4 w-3/4" />
                                <Skeleton className="h-4 w-1/2" />
                            </div>
                        ))}
                    </div>
                </div>
            ) : searchResults ? (
                <div className="mt-8 animate-fade-in">
                    <div className="flex items-center justify-between mb-6">
                        <h2 className="text-xl font-semibold">
                            Results for "{searchQuery}" ({searchResults.movies.length + searchResults.tvShows.length})
                        </h2>
                    </div>

                    <Tabs defaultValue="all" value={activeTab} onValueChange={setActiveTab} className="mb-8">
                        <TabsList>
                            <TabsTrigger value="all">All</TabsTrigger>
                            <TabsTrigger value="movies">Movies</TabsTrigger>
                            <TabsTrigger value="tv">TV Shows</TabsTrigger>
                        </TabsList>

                        <TabsContent value="all" className="mt-6">
                            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                                {getFilteredResults().map((item: any) => (
                                    <SearchResultCard key={item.id} item={item} />
                                ))}
                            </div>
                        </TabsContent>

                        <TabsContent value="movies" className="mt-6">
                            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                                {searchResults.movies.map((movie: any) => (
                                    <SearchResultCard key={movie.id} item={movie} />
                                ))}
                            </div>
                        </TabsContent>

                        <TabsContent value="tv" className="mt-6">
                            <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                                {searchResults.tvShows.map((show: any) => (
                                    <SearchResultCard key={show.id} item={show} />
                                ))}
                            </div>
                        </TabsContent>
                    </Tabs>
                </div>
            ) : (
                <div className="mt-12">
                    {/* Recent Searches */}
                    {recentSearches.length > 0 && (
                        <div className="mb-12 animate-fade-in">
                            <div className="flex items-center justify-between mb-4">
                                <h2 className="text-xl font-semibold flex items-center">
                                    <Clock className="mr-2 h-5 w-5" /> Recent Searches
                                </h2>
                                <Button variant="ghost" size="sm" onClick={clearRecentSearches}>
                                    Clear
                                </Button>
                            </div>
                            <div className="flex flex-wrap gap-2">
                                {recentSearches.map((query, index) => (
                                    <Button
                                        key={index}
                                        variant="outline"
                                        className="rounded-full"
                                        onClick={() => handleRecentSearch(query)}
                                    >
                                        {query}
                                    </Button>
                                ))}
                            </div>
                        </div>
                    )}

                    {/* Popular Searches */}
                    <div className="mb-12 animate-fade-in">
                        <h2 className="text-xl font-semibold flex items-center mb-4">
                            <Trending className="mr-2 h-5 w-5" /> Popular Searches
                        </h2>
                        <div className="flex flex-wrap gap-2">
                            {popularSearches.map((query, index) => (
                                <Button
                                    key={index}
                                    variant="outline"
                                    className="rounded-full"
                                    onClick={() => handlePopularSearch(query)}
                                >
                                    {query}
                                </Button>
                            ))}
                        </div>
                    </div>

                    {/* Browse Categories */}
                    <div className="animate-fade-in">
                        <h2 className="text-xl font-semibold mb-6">Browse Categories</h2>
                        <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                            <CategoryCard
                                title="Movies"
                                icon={<Film className="h-8 w-8" />}
                                count={1200}
                                color="bg-blue-500/10 text-blue-500"
                            />
                            <CategoryCard
                                title="TV Shows"
                                icon={<Tv className="h-8 w-8" />}
                                count={800}
                                color="bg-purple-500/10 text-purple-500"
                            />
                            <CategoryCard
                                title="Top Rated"
                                icon={<Star className="h-8 w-8" />}
                                count={500}
                                color="bg-yellow-500/10 text-yellow-500"
                            />
                            <CategoryCard
                                title="New Releases"
                                icon={<Trending className="h-8 w-8" />}
                                count={350}
                                color="bg-green-500/10 text-green-500"
                            />
                        </div>
                    </div>
                </div>
            )}
        </div>
    )
}

// Search Result Card Component
function SearchResultCard({ item }: { item: any }) {
    return (
        <Card className="overflow-hidden group transition-all duration-300 hover:shadow-lg">
            <div className="relative aspect-[2/3]">
                <img
                    src={item.image || "/placeholder.svg"}
                    alt={item.title}
                    className="absolute inset-0 w-full h-full object-cover transition-transform duration-500 group-hover:scale-105"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300 flex items-end">
                    <div className="p-4 w-full">
                        <Button asChild size="sm" className="w-full gap-2">
                            <Link to={`/content/${item.id}`}>
                                <Play className="h-4 w-4" /> Watch Now
                            </Link>
                        </Button>
                    </div>
                </div>
            </div>
            <CardContent className="p-3">
                <div className="flex items-start justify-between">
                    <div>
                        <h3 className="font-medium line-clamp-1">{item.title}</h3>
                        <p className="text-sm text-muted-foreground">{item.type === "movie" ? item.year : item.years}</p>
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

// Category Card Component
function CategoryCard({
    title,
    icon,
    count,
    color,
}: { title: string; icon: React.ReactNode; count: number; color: string }) {
    return (
        <Card className="overflow-hidden group hover:shadow-md transition-all duration-300">
            <Link to={`/${title.toLowerCase().replace(/\s+/g, "-")}`}>
                <CardContent className="p-6 flex flex-col items-center text-center">
                    <div className={cn("p-4 rounded-full mb-4", color)}>{icon}</div>
                    <h3 className="font-semibold text-lg mb-1">{title}</h3>
                    <p className="text-sm text-muted-foreground">{count} titles</p>
                </CardContent>
            </Link>
        </Card>
    )
}

