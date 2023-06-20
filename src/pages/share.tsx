import PageLayout from '@/components/PageLayout';
import { SetStateAction, useContext, useRef, useState } from 'react';
import { InboxOutlined } from '@ant-design/icons';
import type { UploadProps } from 'antd';
import { message, Upload } from 'antd';
import Dragger from 'antd/es/upload/Dragger';
import { FileContext } from '@/store/context';

/**
 * @function helpPage -  A page responsible for guiding users on various actions
 * @returns tsx
 */
export default function ShareFiles() {
	const { Dragger } = Upload;
	const { onUpdate } = useContext(FileContext);

	const props: UploadProps = {
		name: 'file',
		multiple: true,
		action: 'https://www.mocky.io/v2/5cc8019d300000980a055e76',
		onChange(info) {
			const { status } = info.file;
			if (status !== 'uploading') {
				onUpdate(info.fileList);
				console.log(info.file, info.fileList);
			}
			if (status === 'done') {
				message.success(`${info.file.name} file uploaded successfully.`);
			} else if (status === 'error') {
				message.error(`${info.file.name} file upload failed.`);
			}
		},
		onDrop(e) {
			console.log('Dropped files', e.dataTransfer.files);
		},
	};
	return (
		<>
			<PageLayout pageTitle={'Shared files'} includeSearchBar={false}>
				<Dragger className='h-[500px] block' {...props}>
					<p className='ant-upload-drag-icon'>
						<InboxOutlined rev={undefined} />
					</p>
					<p className='ant-upload-text'>
						Click or drag file to this area to upload
					</p>
					<p className='ant-upload-hint'>
						Support for a single or bulk upload. Strictly prohibited from
						uploading company data or other banned files.
					</p>
				</Dragger>
			</PageLayout>
		</>
	);
}
