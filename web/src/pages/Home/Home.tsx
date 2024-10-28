import { Settings } from "../Settings/Settings";
import { Sidebar } from "../../components/Sidebar/Sidebar";

export const Home = () => {
  return (
    <>
      <div className="flex h-screen w-screen px-4 py-6">
        <Sidebar />

        {/*<div className="ml-6 h-full w-full">
          <Routes>
            <Route path="/settings" element={<Settings />} />
          </Routes>
        </div> */}
        <Settings />
      </div>
    </>
  );
};
