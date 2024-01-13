export const http_fetch = (url: string) => async (input: string) => {
    const result = await fetch(url + input);

    return await result.json();
};
