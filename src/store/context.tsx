import { ReactNode, createContext, useState } from 'react';

type TFileData = {
	size: number;
	type: string;
	name: string;
	status: 'error' | 'done' | 'pending';
};
type TFileContext = {
	fileList: TFileData[];
	onUpdate: Function;
};
export const FileContext = createContext({} as TFileContext);

const FileStore = ({ children }: { children: ReactNode }) => {
	const [fileList, setFileList] = useState<any[]>([]);

	const updateFileList = (data: any) => {
		setFileList(data);
	};

	return (
		<FileContext.Provider
			value={{
				fileList,
				onUpdate: updateFileList,
			}}
		>
			{children}
		</FileContext.Provider>
	);
};

export default FileStore;
