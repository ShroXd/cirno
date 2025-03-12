import { Outlet } from "react-router-dom";
import AppSidebar from "./components/AppSidebar/AppSidebar";

export default function Layout() {
    return (
        <div className="flex min-h-screen">
            <AppSidebar />
            <main className="flex-1">
                <Outlet />
            </main>
        </div>
    )
}
