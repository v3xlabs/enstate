export const http_fetch = (url: string) => async (input: string) => {
    let result = await fetch(url + input);
    return await result.json();
};
