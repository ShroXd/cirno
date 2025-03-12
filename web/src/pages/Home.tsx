import { Link } from "react-router-dom"
import { Button } from "~/components/ui/button"
import { Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle } from "~/components/ui/card"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs"
import { Play, Plus } from "lucide-react"

export default function HomePage() {
    // Mock data for featured content
    const featuredContent = [
        {
            id: "1",
            title: "Interstellar",
            description:
                "A team of explorers travel through a wormhole in space in an attempt to ensure humanity's survival.",
            image: "/placeholder.svg?height=400&width=600",
            type: "movie",
        },
        {
            id: "2",
            title: "Breaking Bad",
            description:
                "A high school chemistry teacher diagnosed with inoperable lung cancer turns to manufacturing and selling methamphetamine.",
            image: "/placeholder.svg?height=400&width=600",
            type: "tv",
        },
        {
            id: "3",
            title: "The Shawshank Redemption",
            description:
                "Two imprisoned men bond over a number of years, finding solace and eventual redemption through acts of common decency.",
            image: "/placeholder.svg?height=400&width=600",
            type: "movie",
        },
        {
            id: "4",
            title: "Game of Thrones",
            description: "Nine noble families fight for control over the lands of Westeros, while an ancient enemy returns.",
            image: "/placeholder.svg?height=400&width=600",
            type: "tv",
        },
    ]

    // Mock data for trending content
    const trendingContent = [
        {
            id: "5",
            title: "Dune",
            description:
                "Feature adaptation of Frank Herbert's science fiction novel about the son of a noble family entrusted with the protection of the most valuable asset in the galaxy.",
            image: "/placeholder.svg?height=300&width=500",
            type: "movie",
        },
        {
            id: "6",
            title: "Succession",
            description: "The Roy family is known for controlling the biggest media and entertainment company in the world.",
            image: "/placeholder.svg?height=300&width=500",
            type: "tv",
        },
        {
            id: "7",
            title: "The Batman",
            description:
                "When a sadistic serial killer begins murdering key political figures in Gotham, Batman is forced to investigate.",
            image: "/placeholder.svg?height=300&width=500",
            type: "movie",
        },
        {
            id: "8",
            title: "Stranger Things",
            description:
                "When a young boy disappears, his mother, a police chief, and his friends must confront terrifying supernatural forces.",
            image: "/placeholder.svg?height=300&width=500",
            type: "tv",
        },
    ]

    return (
        <div className="container mx-auto py-6 px-4 md:px-6">
            <h1 className="text-3xl font-bold mb-6">Welcome to StreamHub</h1>

            <Tabs defaultValue="all" className="mb-8">
                <TabsList>
                    <TabsTrigger value="all">All</TabsTrigger>
                    <TabsTrigger value="movies">Movies</TabsTrigger>
                    <TabsTrigger value="tv">TV Shows</TabsTrigger>
                </TabsList>

                <TabsContent value="all" className="mt-6">
                    <section className="mb-10">
                        <div className="flex items-center justify-between mb-4">
                            <h2 className="text-2xl font-semibold">Featured</h2>
                            <Button variant="link">View All</Button>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                            {featuredContent.map((item) => (
                                <Card key={item.id} className="overflow-hidden">
                                    <div className="relative h-48 w-full">
                                        <Link to={`/content/${item.id}`}>
                                            <img
                                                src={item.image || "/placeholder.svg"}
                                                alt={item.title}
                                                className="absolute inset-0 w-full h-full object-cover"
                                            />
                                            <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
                                            <div className="absolute bottom-3 left-3 flex gap-2">
                                                <Button size="sm" variant="secondary" className="rounded-full">
                                                    <Play className="h-4 w-4 mr-1" /> Play
                                                </Button>
                                                <Button size="icon" variant="outline" className="rounded-full h-8 w-8">
                                                    <Plus className="h-4 w-4" />
                                                </Button>
                                            </div>
                                        </Link>
                                    </div>
                                    <CardHeader className="p-3 pb-0">
                                        <Link to={`/content/${item.id}`}>
                                            <CardTitle className="text-lg">{item.title}</CardTitle>
                                        </Link>
                                    </CardHeader>
                                    <CardContent className="p-3 pt-1">
                                        <CardDescription className="line-clamp-2 text-xs">{item.description}</CardDescription>
                                    </CardContent>
                                    <CardFooter className="p-3 pt-0 text-xs text-muted-foreground">
                                        {item.type === "movie" ? "Movie" : "TV Series"}
                                    </CardFooter>
                                </Card>
                            ))}
                        </div>
                    </section>

                    <section>
                        <div className="flex items-center justify-between mb-4">
                            <h2 className="text-2xl font-semibold">Trending Now</h2>
                            <Button variant="link">View All</Button>
                        </div>

                        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                            {trendingContent.map((item) => (
                                <Card key={item.id} className="overflow-hidden">
                                    <div className="relative h-48 w-full">
                                        <Link to={`/content/${item.id}`}>
                                            <img
                                                src={item.image || "/placeholder.svg"}
                                                alt={item.title}
                                                className="absolute inset-0 w-full h-full object-cover"
                                            />
                                            <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
                                            <div className="absolute bottom-3 left-3 flex gap-2">
                                                <Button size="sm" variant="secondary" className="rounded-full">
                                                    <Play className="h-4 w-4 mr-1" /> Play
                                                </Button>
                                                <Button size="icon" variant="outline" className="rounded-full h-8 w-8">
                                                    <Plus className="h-4 w-4" />
                                                </Button>
                                            </div>
                                        </Link>
                                    </div>
                                    <CardHeader className="p-3 pb-0">
                                        <Link to={`/content/${item.id}`}>
                                            <CardTitle className="text-lg">{item.title}</CardTitle>
                                        </Link>
                                    </CardHeader>
                                    <CardContent className="p-3 pt-1">
                                        <CardDescription className="line-clamp-2 text-xs">{item.description}</CardDescription>
                                    </CardContent>
                                    <CardFooter className="p-3 pt-0 text-xs text-muted-foreground">
                                        {item.type === "movie" ? "Movie" : "TV Series"}
                                    </CardFooter>
                                </Card>
                            ))}
                        </div>
                    </section>
                </TabsContent>

                <TabsContent value="movies" className="mt-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                        {[...featuredContent, ...trendingContent]
                            .filter((item) => item.type === "movie")
                            .map((item) => (
                                <Card key={item.id} className="overflow-hidden">
                                    <div className="relative h-48 w-full">
                                        <Link to={`/content/${item.id}`}>
                                            <img
                                                src={item.image || "/placeholder.svg"}
                                                alt={item.title}
                                                className="absolute inset-0 w-full h-full object-cover"
                                            />
                                            <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
                                            <div className="absolute bottom-3 left-3 flex gap-2">
                                                <Button size="sm" variant="secondary" className="rounded-full">
                                                    <Play className="h-4 w-4 mr-1" /> Play
                                                </Button>
                                                <Button size="icon" variant="outline" className="rounded-full h-8 w-8">
                                                    <Plus className="h-4 w-4" />
                                                </Button>
                                            </div>
                                        </Link>
                                    </div>
                                    <CardHeader className="p-3 pb-0">
                                        <Link to={`/content/${item.id}`}>
                                            <CardTitle className="text-lg">{item.title}</CardTitle>
                                        </Link>
                                    </CardHeader>
                                    <CardContent className="p-3 pt-1">
                                        <CardDescription className="line-clamp-2 text-xs">{item.description}</CardDescription>
                                    </CardContent>
                                </Card>
                            ))}
                    </div>
                </TabsContent>

                <TabsContent value="tv" className="mt-6">
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                        {[...featuredContent, ...trendingContent]
                            .filter((item) => item.type === "tv")
                            .map((item) => (
                                <Card key={item.id} className="overflow-hidden">
                                    <div className="relative h-48 w-full">
                                        <Link to={`/content/${item.id}`}>
                                            <img
                                                src={item.image || "/placeholder.svg"}
                                                alt={item.title}
                                                className="absolute inset-0 w-full h-full object-cover"
                                            />
                                            <div className="absolute inset-0 bg-gradient-to-t from-black/80 to-transparent" />
                                            <div className="absolute bottom-3 left-3 flex gap-2">
                                                <Button size="sm" variant="secondary" className="rounded-full">
                                                    <Play className="h-4 w-4 mr-1" /> Play
                                                </Button>
                                                <Button size="icon" variant="outline" className="rounded-full h-8 w-8">
                                                    <Plus className="h-4 w-4" />
                                                </Button>
                                            </div>
                                        </Link>
                                    </div>
                                    <CardHeader className="p-3 pb-0">
                                        <Link to={`/content/${item.id}`}>
                                            <CardTitle className="text-lg">{item.title}</CardTitle>
                                        </Link>
                                    </CardHeader>
                                    <CardContent className="p-3 pt-1">
                                        <CardDescription className="line-clamp-2 text-xs">{item.description}</CardDescription>
                                    </CardContent>
                                </Card>
                            ))}
                    </div>
                </TabsContent>
            </Tabs>
        </div>
    )
}
