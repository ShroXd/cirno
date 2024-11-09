import useSWR from "swr";
import { TVSeriesDTO } from "../../bindings/TVSeriesDTO";

export const Library = () => {
  const { data, error, isLoading } = useSWR<TVSeriesDTO[]>(
    "/media-library/series",
    (url: string) => fetch(url).then((res) => res.json()),
  );

  if (error) return <div>Error: {error.message}</div>;
  if (isLoading) return <div>Loading...</div>;

  return (
    <div>
      <h1>Library</h1>
      <ul>
        {data?.map((series) => (
          <li className="text-2xl text-gray-800" key={series.title}>
            {series.title}
          </li>
        ))}
      </ul>
    </div>
  );
};
