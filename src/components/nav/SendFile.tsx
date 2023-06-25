import Image from "next/image";

export default function SendFileComponent() {
  return (
    <form action="" style={{ background: "", padding: "4px", height: "260px" }}>
      <div className="flex flex-col align-center justify-center">
        <label htmlFor="connectionID " className="text-gray-600 sr-only">
          connection ID
        </label>
        <div className="flex items-center my-4 justify-center mx-auto">
          <Image
            src="/icons/Computer-Tablet-and-Phone-Vectors---1.0.0.svg"
            alt="recieve files"
            className="w-[150px]"
            width={400}
            height={400}
          />
        </div>
        <input
          type="text"
          maxLength={6}
          name="connectionID"
          placeholder="enter connection ID"
          id="connectionID"
          className="border-2 placeholder:text-small my-0 w-2/3 mt-10 mx-auto border-gray-300 rounded-md p-2"
        />
      </div>
    </form>
  );
}
