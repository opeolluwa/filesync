import { goToPrevPage, goToNextPage } from "@/utils";
import { ChevronLeftIcon, ChevronRightIcon } from "@heroicons/react/24/outline";
import SearchBar from "../../Search";
import PageTitle from "../../PageTitle";
import React from "react";

interface Props {
  children: React.ReactNode;
  pageTitle: string;
  includeSearchBar: boolean;
  searchBarText?: string;
  includePageTitle?: boolean;
}

export default function PageLayout({
  children,
  pageTitle,
  includeSearchBar = false,
  searchBarText = "search",
  includePageTitle = true,
}: Props) {
  return (
    <>
      {includePageTitle && (
        <div className="hidden sm:flex mb-10 font-medium dark:text-gray-300 justify-between items-center">
          <ChevronLeftIcon
            className="text-gray-400 w-6 h-6 text-medium cursor-pointer mr-8"
            onClick={goToPrevPage}
          />
          <PageTitle styles="capitalize" title={pageTitle}></PageTitle>
          <ChevronRightIcon
            className="text-gray-400 w-6 h-6 text-medium cursor-pointer mr-8"
            onClick={goToNextPage}
          />
        </div>
      )}
      {includeSearchBar && (
        <SearchBar
          onSearch={function (city: string): void {
            throw new Error("Function not implemented.");
          }}
          placeholder={searchBarText}
        />
      )}

      {includePageTitle && (
        <div className="dark:text-gray-400 ">{children}</div>
      )}
      <div className="dark:text-gray-400 mt-10">{children}</div>
    </>
  );
}
