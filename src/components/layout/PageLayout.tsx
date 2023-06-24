import { goToPrevPage, goToNextPage } from "@/utils";
import { ChevronLeftIcon, ChevronRightIcon } from "@heroicons/react/24/outline";
import SearchBar from "../SearchBar";
import PageTitle from "../PageTitle";

interface Props {
  children: React.ReactNode;
  pageTitle: string;
  includeSearchBar: boolean;
}

export default function QuickAccessLayout({
  children,
  pageTitle,
  includeSearchBar = false,
}: Props) {
  return (
    <>
      {/** page title  and navigation icons  */}
      <div className="hidden mb-10 font-medium dark:text-gray-300 justify-between items-center">
        <ChevronLeftIcon
          className="text-gary-300 dark:text-gray-200 dark:hover:text-shilo-700 w-6 h-6 text-medium cursor-pointer mr-8"
          onClick={goToPrevPage}
        />
        <h2 className="text-xl font-bold text-gray-700 dark:text-gray-300">
          {pageTitle}
        </h2>
        <ChevronRightIcon
          className="text-gary-300 dark:text-gray-200 dark:hover:text-shilo-700 w-6 h-6 text-medium cursor-pointer mr-8"
          onClick={goToNextPage}
        />
      </div>

      {/** search bar  */}
      {includeSearchBar && (
        <SearchBar
          onSearch={function (city: string): void {
            throw new Error("Function not implemented.");
          }}
        />
      )}

      {/** the view/children nodes  */}
      <div className="dark:text-gray-400 mt-10">
        {/**inject child element  */}
        {children}
      </div>
    </>
  );
}
