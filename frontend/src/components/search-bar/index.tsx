import React from "react";

const SearchBar = () => {
	return (
	<div className="bg-dark-100 p-1 m-4 rounded-md w-full text-xl min-w-fit">
			<select className="bg-dark-100 h-full p-2 outline-none">
				<option>euw</option>
			</select>
		<input className="placeholder:capitalize bg-dark-100 p-2 outline-none" placeholder="search a summoner..." />
	</div>
)
}

export default SearchBar;
