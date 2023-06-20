import Layout from '@/components/AppLayout';
import FileStore from '@/store/context';
import '@/styles/globals.css';
import type { AppProps } from 'next/app';

export default function App({ Component, pageProps }: AppProps) {
	return (
		<FileStore>
			<Layout>
				<Component {...pageProps} />
			</Layout>
		</FileStore>
	);
}
