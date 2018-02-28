import {Coordinates, Photo} from './interfaces';

interface PhotosResponseBody {
  photos: Photo[];
}

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
    method: 'PUT'
  };

  return fetch(url, init).then(response => {
    if (!response.ok) {
      throw new Error(`Failed to ${init.method} ${url} with body ${init.body}`);
    }
  });
}

function setPhotoSrc(photo: Photo) {
  return Object.assign(photo, {
    src: `/thumbnail?path=${encodeURIComponent(
      photo.path
    )}&maxWidth=300&maxHeight=300`
  });
}

function mapPhotos(responseBody: PhotosResponseBody) {
  return responseBody.photos.map(setPhotoSrc);
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
  return get('/photos').then(mapPhotos);
}

export function getFilteredPhotos() {
  return get('/photos?filter').then(mapPhotos);
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
