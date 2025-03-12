import { useParams, Link } from "react-router-dom"
import { Button } from "~/components/ui/button"
import { Tabs, TabsContent, TabsList, TabsTrigger } from "~/components/ui/tabs"
import { Badge } from "~/components/ui/badge"
import { Play, Plus, Star, Clock, Calendar } from "lucide-react"

// This would come from your database or API in a real app
const getContentById = (id: string) => {
    const allContent = [
        {
            id: "1",
            title: "Interstellar",
            description:
                "A team of explorers travel through a wormhole in space in an attempt to ensure humanity's survival.",
            longDescription:
                "When Earth becomes uninhabitable in the future, a farmer and ex-NASA pilot, Joseph Cooper, is tasked to pilot a spacecraft, along with a team of researchers, to find a new planet for humans. The mission, however, takes an unexpected turn when they travel through a wormhole near Saturn and find themselves in a different galaxy with drastically altered physics and dimensions of space and time.",
            image: "/placeholder.svg?height=600&width=1000",
            type: "movie",
            releaseYear: 2014,
            duration: "2h 49m",
            rating: 8.7,
            genres: ["Sci-Fi", "Adventure", "Drama"],
            director: "Christopher Nolan",
            cast: ["Matthew McConaughey", "Anne Hathaway", "Jessica Chastain"],
        },
        {
            id: "2",
            title: "Breaking Bad",
            description:
                "A high school chemistry teacher diagnosed with inoperable lung cancer turns to manufacturing and selling methamphetamine.",
            longDescription:
                "Set in Albuquerque, New Mexico, between 2008 and 2010, Breaking Bad follows Walter White, a meek high school chemistry teacher who transforms into a ruthless player in the local methamphetamine drug trade, driven by a desire to financially provide for his family after being diagnosed with terminal lung cancer.",
            image: "/placeholder.svg?height=600&width=1000",
            type: "tv",
            releaseYear: 2008,
            endYear: 2013,
            rating: 9.5,
            genres: ["Crime", "Drama", "Thriller"],
            creator: "Vince Gilligan",
            cast: ["Bryan Cranston", "Aaron Paul", "Anna Gunn"],
            seasons: [
                {
                    number: 1,
                    episodes: [
                        { id: "2-1-1", number: 1, title: "Pilot", duration: "58m" },
                        { id: "2-1-2", number: 2, title: "Cat's in the Bag...", duration: "48m" },
                        { id: "2-1-3", number: 3, title: "...And the Bag's in the River", duration: "48m" },
                        { id: "2-1-4", number: 4, title: "Cancer Man", duration: "48m" },
                        { id: "2-1-5", number: 5, title: "Gray Matter", duration: "48m" },
                        { id: "2-1-6", number: 6, title: "Crazy Handful of Nothin'", duration: "47m" },
                        { id: "2-1-7", number: 7, title: "A No-Rough-Stuff-Type Deal", duration: "47m" },
                    ],
                },
                {
                    number: 2,
                    episodes: [
                        { id: "2-2-1", number: 1, title: "Seven Thirty-Seven", duration: "47m" },
                        { id: "2-2-2", number: 2, title: "Grilled", duration: "48m" },
                        { id: "2-2-3", number: 3, title: "Bit by a Dead Bee", duration: "47m" },
                        { id: "2-2-4", number: 4, title: "Down", duration: "47m" },
                        { id: "2-2-5", number: 5, title: "Breakage", duration: "47m" },
                    ],
                },
                {
                    number: 3,
                    episodes: [
                        { id: "2-3-1", number: 1, title: "No Más", duration: "47m" },
                        { id: "2-3-2", number: 2, title: "Caballo Sin Nombre", duration: "47m" },
                        { id: "2-3-3", number: 3, title: "I.F.T.", duration: "47m" },
                        { id: "2-3-4", number: 4, title: "Green Light", duration: "47m" },
                        { id: "2-3-5", number: 5, title: "Más", duration: "47m" },
                    ],
                },
            ],
        },
        {
            id: "3",
            title: "The Shawshank Redemption",
            description:
                "Two imprisoned men bond over a number of years, finding solace and eventual redemption through acts of common decency.",
            longDescription:
                "Andy Dufresne, a successful banker, is arrested for the murders of his wife and her lover, and is sentenced to life imprisonment at the Shawshank prison. He becomes the most unconventional prisoner and befriends a fellow prisoner, Ellis Redding.",
            image: "/placeholder.svg?height=600&width=1000",
            type: "movie",
            releaseYear: 1994,
            duration: "2h 22m",
            rating: 9.3,
            genres: ["Drama"],
            director: "Frank Darabont",
            cast: ["Tim Robbins", "Morgan Freeman", "Bob Gunton"],
        },
        {
            id: "4",
            title: "Game of Thrones",
            description: "Nine noble families fight for control over the lands of Westeros, while an ancient enemy returns.",
            longDescription:
                "In the mythical continent of Westeros, several powerful families fight for control of the Seven Kingdoms. As conflict erupts in the kingdoms of men, an ancient enemy rises once again to threaten them all. Meanwhile, the last heirs of a recently usurped dynasty plot to take back their homeland from across the Narrow Sea.",
            image: "/placeholder.svg?height=600&width=1000",
            type: "tv",
            releaseYear: 2011,
            endYear: 2019,
            rating: 9.2,
            genres: ["Action", "Adventure", "Drama"],
            creator: "David Benioff, D.B. Weiss",
            cast: ["Emilia Clarke", "Peter Dinklage", "Kit Harington"],
            seasons: [
                {
                    number: 1,
                    episodes: [
                        { id: "4-1-1", number: 1, title: "Winter Is Coming", duration: "62m" },
                        { id: "4-1-2", number: 2, title: "The Kingsroad", duration: "56m" },
                        { id: "4-1-3", number: 3, title: "Lord Snow", duration: "58m" },
                        { id: "4-1-4", number: 4, title: "Cripples, Bastards, and Broken Things", duration: "56m" },
                        { id: "4-1-5", number: 5, title: "The Wolf and the Lion", duration: "55m" },
                    ],
                },
                {
                    number: 2,
                    episodes: [
                        { id: "4-2-1", number: 1, title: "The North Remembers", duration: "53m" },
                        { id: "4-2-2", number: 2, title: "The Night Lands", duration: "54m" },
                        { id: "4-2-3", number: 3, title: "What Is Dead May Never Die", duration: "53m" },
                        { id: "4-2-4", number: 4, title: "Garden of Bones", duration: "51m" },
                        { id: "4-2-5", number: 5, title: "The Ghost of Harrenhal", duration: "55m" },
                    ],
                },
            ],
        },
    ]

    return allContent.find((item) => item.id === id)
}

export default function ContentDetailPage() {
    const { id } = useParams<{ id: string }>()
    const content = getContentById(id || "")

    if (!content) {
        return (
            <div className="container mx-auto py-12 px-4 text-center">
                <h1 className="text-3xl font-bold mb-4">Content Not Found</h1>
                <p>The content you're looking for doesn't exist or has been removed.</p>
            </div>
        )
    }

    const isMovie = content.type === "movie"

    return (
        <div className="container mx-auto py-6 px-4 md:px-6 overflow-y-auto h-screen">
            {/* Hero Section */}
            <div className="relative w-full h-[50vh] mb-8 rounded-xl overflow-hidden">
                <img
                    src={content.image || "/placeholder.svg"}
                    alt={content.title}
                    className="absolute inset-0 w-full h-full object-cover"
                />
                <div className="absolute inset-0 bg-gradient-to-t from-background via-background/80 to-transparent" />

                <div className="absolute bottom-0 left-0 right-0 p-6 md:p-8">
                    <div className="flex flex-col gap-4 max-w-3xl">
                        <div className="flex flex-wrap gap-2">
                            {content.genres.map((genre) => (
                                <Badge key={genre} variant="secondary">
                                    {genre}
                                </Badge>
                            ))}
                        </div>

                        <h1 className="text-3xl md:text-5xl font-bold">{content.title}</h1>

                        <div className="flex flex-wrap items-center gap-4 text-sm text-muted-foreground">
                            <div className="flex items-center">
                                <Star className="w-4 h-4 mr-1 fill-yellow-400 text-yellow-400" />
                                <span>{content.rating}/10</span>
                            </div>

                            <div className="flex items-center">
                                <Calendar className="w-4 h-4 mr-1" />
                                <span>
                                    {isMovie ? content.releaseYear : `${content.releaseYear} - ${content.endYear || "Present"}`}
                                </span>
                            </div>

                            {isMovie && (
                                <div className="flex items-center">
                                    <Clock className="w-4 h-4 mr-1" />
                                    <span>{content.duration}</span>
                                </div>
                            )}
                        </div>

                        <div className="flex gap-3 mt-2">
                            <Button className="gap-2" asChild>
                                <Link to={`/watch/${isMovie ? content.id : content?.seasons?.[0]?.episodes?.[0]?.id}`}>
                                    <Play className="w-4 h-4" />
                                    {isMovie ? "Watch Movie" : "Watch Latest"}
                                </Link>
                            </Button>
                            <Button variant="outline" className="gap-2">
                                <Plus className="w-4 h-4" />
                                Add to Playlist
                            </Button>
                        </div>
                    </div>
                </div>
            </div>

            {/* Content Details */}
            <div className="grid grid-cols-1 lg:grid-cols-3 gap-8 mb-8">
                <div className="lg:col-span-2">
                    <h2 className="text-2xl font-semibold mb-4">Overview</h2>
                    <p className="text-muted-foreground mb-6">{content.longDescription}</p>

                    {!isMovie && (
                        <div className="mt-8">
                            <h2 className="text-2xl font-semibold mb-4">Seasons & Episodes</h2>
                            <Tabs defaultValue={`season-${content.seasons?.[0]?.number}`}>
                                <TabsList className="mb-4 flex flex-wrap">
                                    {content.seasons?.map((season) => (
                                        <TabsTrigger key={season.number} value={`season-${season.number}`}>
                                            Season {season.number}
                                        </TabsTrigger>
                                    ))}
                                </TabsList>

                                {content.seasons?.map((season) => (
                                    <TabsContent key={season.number} value={`season-${season.number}`}>
                                        <div className="space-y-4">
                                            {season.episodes.map((episode) => (
                                                <div
                                                    key={episode.number}
                                                    className="flex items-center justify-between p-4 rounded-lg border hover:bg-muted/50 transition-colors"
                                                >
                                                    <div className="flex items-center gap-4">
                                                        <div className="flex items-center justify-center w-10 h-10 rounded-full bg-primary/10 text-primary font-medium">
                                                            {episode.number}
                                                        </div>
                                                        <div>
                                                            <h4 className="font-medium">{episode.title}</h4>
                                                            <p className="text-sm text-muted-foreground">{episode.duration}</p>
                                                        </div>
                                                    </div>
                                                    <Button size="sm" variant="ghost" asChild>
                                                        <Link to={`/watch/${episode.id}`}>
                                                            <Play className="w-4 h-4" />
                                                        </Link>
                                                    </Button>
                                                </div>
                                            ))}
                                        </div>
                                    </TabsContent>
                                ))}
                            </Tabs>
                        </div>
                    )}
                </div>

                <div>
                    <div className="rounded-lg border p-4">
                        <h3 className="text-lg font-medium mb-4">Details</h3>
                        <dl className="space-y-4">
                            {isMovie ? (
                                <>
                                    <div>
                                        <dt className="text-sm text-muted-foreground">Director</dt>
                                        <dd className="mt-1">{content.director}</dd>
                                    </div>
                                </>
                            ) : (
                                <>
                                    <div>
                                        <dt className="text-sm text-muted-foreground">Creator</dt>
                                        <dd className="mt-1">{content.creator}</dd>
                                    </div>
                                    <div>
                                        <dt className="text-sm text-muted-foreground">Seasons</dt>
                                        <dd className="mt-1">{content.seasons?.length}</dd>
                                    </div>
                                </>
                            )}

                            <div>
                                <dt className="text-sm text-muted-foreground">Cast</dt>
                                <dd className="mt-1">
                                    <ul className="space-y-1">
                                        {content.cast.map((actor) => (
                                            <li key={actor}>{actor}</li>
                                        ))}
                                    </ul>
                                </dd>
                            </div>

                            <div>
                                <dt className="text-sm text-muted-foreground">Genres</dt>
                                <dd className="mt-1 flex flex-wrap gap-1">
                                    {content.genres.map((genre) => (
                                        <Badge key={genre} variant="outline">
                                            {genre}
                                        </Badge>
                                    ))}
                                </dd>
                            </div>
                        </dl>
                    </div>
                </div>
            </div>

            {/* Related Content Section */}
            <div className="mt-12">
                <h2 className="text-2xl font-semibold mb-6">You May Also Like</h2>
                <div className="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-6">
                    {/* This would be dynamically generated based on related content */}
                    {[1, 2, 3, 4].map((i) => (
                        <div key={i} className="rounded-lg overflow-hidden border">
                            <div className="relative aspect-video">
                                <img
                                    src="/placeholder.svg?height=300&width=500"
                                    alt="Related content"
                                    className="absolute inset-0 w-full h-full object-cover"
                                />
                            </div>
                            <div className="p-3">
                                <h3 className="font-medium truncate">Related Title {i}</h3>
                                <p className="text-xs text-muted-foreground mt-1">
                                    {isMovie ? "Movie" : "TV Series"} • {2020 + i}
                                </p>
                            </div>
                        </div>
                    ))}
                </div>
            </div>
        </div>
    )
}
