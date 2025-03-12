import AppSidebar from "./components/AppSidebar/AppSidebar";

export default function Layout() {
    return (
        <div className="flex min-h-screen">
            <AppSidebar />
            <main className="flex-1">
                Main content
            </main>
        </div>
    )
}
