import { createElement, useEffect } from "react";
import { useLocation, useNavigate } from "react-router-dom";
import {
  FolderIcon,
  Cog6ToothIcon,
  SparklesIcon,
} from "@heroicons/react/24/outline";
import MovieIcon from "/movie.svg";

export const Sidebar = () => {
  const navigate = useNavigate();
  const location = useLocation();

  useEffect(() => {
    console.log(location);
  }, [location]);

  const data = [
    {
      value: "library",
      icon: FolderIcon,
      path: "/",
    },
    {
      value: "settings",
      icon: Cog6ToothIcon,
      path: "/settings",
    },
    {
      value: "test",
      icon: SparklesIcon,
      path: "/test",
    },
  ];

  return (
    <div className="flex flex-col items-center gap-2 px-2 py-5 border w-20 border-gray-700 rounded-xl bg-gray-900 shadow-xl">
      <img src={MovieIcon} alt="Movie Icon" className="w-11 h-11" />
      <div className="flex flex-col items-center justify-center flex-1 gap-2">
        {data.map(({ value, icon, path }) => (
          <button
            className={`px-3 py-3 rounded-xl focus:outline-none hover:border-transparent transition-all ease-in-out duration-300 shadow-inner s ${
              location.pathname === path ? "bg-gray-800" : "bg-transparent"
            }`}
            key={value}
            onClick={() => navigate(path)}
          >
            {createElement(icon, { className: "w-6 h-6 text-gray-50" })}
          </button>
        ))}
      </div>
    </div>
  );
};
