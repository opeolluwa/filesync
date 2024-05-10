
import PageTitle from "../components/PageTitle";
import SearchBar from "../components/Search";
import {
  ArchiveBoxIcon,
  CloudArrowDownIcon,
  DocumentDuplicateIcon,
  FilmIcon,
  MusicalNoteIcon,
  PhotoIcon,
} from "@heroicons/react/24/outline";



// seed the files 
const transferHistory = {
  data: [
    {
      fileName: "file1",
      fileSize: "1000",
      date: "2021-09-12",
    },
    {
      fileName: "file2",
      fileSize: "2000",
      date: "2021-09-12",
    },
    {
      fileName: "file3",
      fileSize: "3000",
      date: "2021-09-12",
    },
    {
      fileName: "file4",
      fileSize: "4000",
      date: "2021-09-12",
    },
    {
      fileName: "file5",
      fileSize: "5000",
      date: "2021-09-12",
    },
  ],
};
const isLoading = false;
const tabs = [
  {
    name: "Pictures",
    icon: (
      <PhotoIcon className="rounded-lg my-4 mx-2 flex w-[40px] text-gray-100 " />
    ),
  },
  {
    name: "Audio",
    icon: (
      <MusicalNoteIcon className="rounded-lg my-4 mx-2 flex w-[40px]   text-gray-100 " />
    ),
  },
  {
    name: "Documents",
    icon: (
      <DocumentDuplicateIcon className="rounded-lg my-4 mx-2 flex w-[40px]   text-gray-100 " />
    ),
  },
  {
    name: "Videos",
    icon: (
      <FilmIcon className="rounded-lg my-4 mx-2 flex w-[40px]   text-gray-100 " />
    ),
  },
  {
    name: "Zipped",
    icon: (
      <ArchiveBoxIcon className="rounded-sm my-4 mx-2 flex w-[40px]  text-gray-100 " />
    ),
  },
  {
    name: "Downloads",
    icon: (
      <CloudArrowDownIcon className="rounded-sm my-4 mx-2 flex w-[40px]  text-gray-100 " />
    ),
  },
];

export default function Index() {
  return (
    <div
      style={{
        height: "100vh",
      }}
    >
      <section>
        <SearchBar
          onSearch={function (): void {
            throw new Error("Function not implemented.");
          }}
          placeholder={"search files"}
        />
      </section>

      <section className="my-4">
        <PageTitle styles="mt-12" title={"Quick Access"} />
        <ul className="grid grid-flow-col col-5 gap-8 items-center justify-start mt-4  overflow-x-scroll">
          {tabs.map((tab, index) => (
            <li
              key={index}
              className="flex flex-col items-center justify-evenly "
            >
              <a
                href={"quick-access/" + tab.name.toLowerCase()}
                className="rounded-[12px] px-3 hover:bg-[#3074f5]"
                style={{
                  // backgroundColor: "#3074f5",
                  backgroundColor: "#578EF7",
                }}
              >
                <div className="hover:brightness-25 sepia-0">{tab.icon}</div>
              </a>
              <span className="text-gray-600 block mt-2 text-small">
                {tab.name}
              </span>
            </li>
          ))}
        </ul>
      </section>

      <section className="my-16">
        <h2 className="flex justify-between mt-24 mb-4 ">
          <span className=" font-medium text-gray-400">
            <PageTitle styles="" title={"Recent Files"} />
          </span>
          <a href="/history" className="text-gray-500 text-violet-600 ">
            view all
          </a>
        </h2>
        <div className="relative overflow-x-auto bg-white rounded-[24px] shadow-sm px-4 py-8">
          <table className="w-full text-sm text-left">
            <thead className="text-gray-500">
              <tr>
                <th scope="col" className="px-6 py-3 rounded-l-lg">
                  Name
                </th>
                <th scope="col" className="px-6 py-3">
                  Size
                </th>
                <th scope="col" className="px-6 py-3 rounded-r-lg">
                  Last Modified
                </th>
              </tr>
            </thead>
            <tbody className="text-gray-500 transition-all delay-75 ease-in">
              {isLoading ? (
                // <Spin indicator={antIcon} />
                <span>heheeh</span>
              ) : (
                transferHistory?.data
                  ?.sort(
                    (a, b) =>
                      new Date(a.date).getTime() - new Date(b.date).getTime()
                  )
                  .reverse()
                  .slice(0, 5)
                  .map((file, index) => (
                    <tr key={index}>
                      <td className="px-6 py-4">{file.fileName}</td>
                      <td className="px-6 py-4">
                        20kb
                        {/* {computeFileSize(Number(file.fileSize))} */}
                      </td>
                      <td className="px-6 py-4">{file.date}</td>
                    </tr>
                  ))
              )}
            </tbody>
          </table>
        </div>
      </section>
      <section className="my-16 hidden">
        <h2 className="flex justify-between mt-20 mb-8 ">
          <span className=" font-medium dark:text-gray-400">Recent Files</span>
          <span className="text-gray-500 dark:text-violet">view all</span>
        </h2>
      </section>
    </div>
  );
}
