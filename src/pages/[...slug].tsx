"use client";

import { useRouter } from "next/router";
import { useEffect, useState } from "react";
import { useSearchParams } from "next/navigation";
import { FileInterface } from "@/components/thumbnail";
import { AppData } from "@/types";
import { invoke } from "@tauri-apps/api";
import LoaderCircle from "@/components/loaders/LoaderCircle";

export default function PreviewMediaPage() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const filePath = searchParams.get("filePath");
  const isFolder = searchParams.get("isFolder");
  const fileType = searchParams.get("fileType");

 const [data, setData] = useState(null);
 const [isLoading, setLoading] = useState(false);

 // get the data from the application core
 useEffect(() => {
   setLoading(true);
   invoke("read_dir", { path: "documents" }).then((res) => {
     setData(res as any);
     setLoading(false);
   });
 }, []);

 // typecast the response into AppData type
 const fetchedDocuments = data as unknown as AppData<Array<FileInterface>>;
 if (isLoading) {
   return (
     <>
       <LoaderCircle />
       <h2 className="font-xl font-bold mt-8">Loading...</h2>
       <p className="leading-5 text-gray-400">
         Please wait while we load your documents. This might take a while.
       </p>
     </>
   );
 }

  
  return (
    <div>
      <h1> file path: {filePath} </h1>
      <h1>file type {fileType}</h1>
      <h1>
       is folder {isFolder}
      </h1>
      Lorem ipsum dolor sit amet consectetur adipisicing elit. Repellat iste
      sequi aliquam aliquid! Iusto doloribus veritatis facere aliquam mollitia
      consequuntur hic, nostrum, fugiat pariatur incidunt, modi voluptatum illo
      doloremque maxime?
    </div>
  );
}
