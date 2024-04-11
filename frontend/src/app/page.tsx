import SearchBar from "@/components/search-bar";

export default function Home() {
  return (
    <main className="w-screen h-screen flex items-center justify-center bg-dark-200 flex flex-col">
		<div className="min-w-fit flex flex-col items-center">
			<h1 className="text-5xl uppercase font-semibold">nashor.gg</h1>
			<SearchBar />
		</div>
	</main>
  );
}
