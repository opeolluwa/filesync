
/**
 * 
 * @function computeFileSize - compute file size to human readable format
 * @param size - file size in byte
 * @returns file size and extension e.g 3.5 MB
 */

export function computeFileSize(size: number) {
    if (size > 1024 * 1024 * 1024 * 1024) {
        return (size / (1024 * 1024 * 1024 * 1024)).toFixed(1).toString() + " TB";
    }
    else if (size > 1024 * 1024 * 1024) {
        return (size / (1024 * 1024 * 1024)).toFixed(1).toString() + " GB";
    } else if (size > 1024 * 1024) {
        return (size / (1024 * 1024)).toFixed(1).toString() + " MB";
    } else if (size > 1024) {
        return (size / 1024).toFixed(1).toString() + " KB";
    } else {
        return size.toString() + " B";
    }
}

// go bak to prev page
export function goToPrevPage() {
    window.history.back()

}
// navigate to the next page
export function goToNextPage() {
    window.history.forward()

}

/**
 * @function gotoPage - to to the specified page
 * @param routePath a string of the route path relative to the index route 
 */

export interface AppRouterInterface {
    routePath: string
}
export function goToPage({ routePath }: any) {
    if ( typeof window !== 'undefined') {
        // browser code
        window.location.href = routePath;
    }

}