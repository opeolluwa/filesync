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

export default function PageLayout({ children }: Props) {
  return (
    <>
      <div className={"text-gray-400 "}>{children}</div>
    </>
  );
}
