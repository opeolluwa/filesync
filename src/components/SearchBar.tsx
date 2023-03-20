import { MagnifyingGlassIcon } from "@heroicons/react/24/outline";
import { useState } from "react";

/**
 *  types definition for the search filed 
 * takes the keyword TODO and the function to execute which will be imported from Rust core
 */
interface Props {
  onSearch: (city: string) => void;
}


export default function SearchBar({ onSearch }: Props) {
  const [city, setCity] = useState('');

  function handleSubmit(e: { preventDefault: () => void; }) {
    e.preventDefault();
    onSearch(city);
  }

  return (
    <form onSubmit={handleSubmit} style={{
      position: 'relative'
    }}>
      <label htmlFor="search box" className="sr-only"> search files</label>
      <input
        className="px-4 py-2 rounded block w-full border-gray-400 dark:border-gray-800 bg-[#fafbfd] dark:bg-[#1a1b1b]"
        type="search"
        placeholder="browse files"
        value={city}
        onChange={e => setCity(e.target.value)}
      />
      <button type="submit" style={{
        position: 'absolute',
        right: '0.5rem',
        top: '0.5rem'
      }}>
        <span className="sr-only">search</span>
        <MagnifyingGlassIcon className="w-6 h-6 text-gray-600" />
      </button>
    </form>
  );
}