import { Coordinates } from './interfaces';

function get(url: string) {
  return fetch(url).then(response => {
    if (response.ok) {
      return response.json();
    }
    throw new Error(`Failed to GET ${url}`);
  });
}

function put(url: string, body: any) {
  const init = {
    body: JSON.stringify(body),
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json'
    }
  };

  return fetch(url, init).then(response => {
    if (!response.ok) {
      throw new Error(`Failed to ${init.method} ${url} with body ${init.body}`);
    }
  });
}

export function getRootPath() {
  return get('/rootPath');
}

export function getNewRootPath() {
  return get('/rootPath/new');
}

export function getLocationHistoryPath() {
  return get('/locationHistoryPath');
}

export function getNewLocationHistory() {
  return get('/locationHistory/new');
}

export function getInterpolate() {
  return get('/interpolate');
}

export function putInterpolate(interpolate: boolean) {
  return put('/interpolate', { interpolate });
}

export function getPhotos() {
  return get('/photos').then(body => body.photos);
}

export function getFilteredPhotos() {
  return get('/photos?filter=true').then(body => body.photos);
}

export function writeCoordinates(path: string, coordinates: Coordinates) {
  return put(`/location?path=${encodeURIComponent(path)}`, coordinates);
}

export function getLocations(startIndex: number, endIndex: number) {
  return get(`/locations?start=${startIndex}&end=${endIndex}`).then(
    responseBody => responseBody.locations
  );
}

export function getLocation(path: string) {
  return get(`/location?path=${encodeURIComponent(path)}`);
}
