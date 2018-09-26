import * as React from 'react';
import * as renderer from 'react-test-renderer';
import { PhotosGrid } from '../../src/gui/js/photos-grid';

describe('PhotosGrid', () => {
  let mockGetAndStoreLocations: jest.Mock;
  let mockHandlePhotoSelect: jest.Mock;
  let filteredPhotosGrid: renderer.ReactTestRenderer;

  const photos = [
    {
      height: 5,
      width: 10,
      loaded: false,
      path: 'path',
      src: 'source'
    },
    {
      height: 5,
      width: 10,
      loaded: true,
      path: 'path',
      src: 'source'
    }
  ];

  beforeAll(() => {
    mockGetAndStoreLocations = jest.fn();
    mockHandlePhotoSelect = jest.fn();

    filteredPhotosGrid = renderer.create(
      <PhotosGrid
        photos={photos}
        getAndStoreLocations={mockGetAndStoreLocations}
        handlePhotoSelect={mockHandlePhotoSelect}
      />
    );
  });

  beforeEach(() => {
    mockGetAndStoreLocations.mockClear();
    mockHandlePhotoSelect.mockClear();
  });

  test('renders a grid of photo thumbnails and a filter checkbox and label', () => {
    const photosGrid = renderer
      .create(
        <PhotosGrid
          photos={photos}
          getAndStoreLocations={mockGetAndStoreLocations}
          handlePhotoSelect={mockHandlePhotoSelect}
        />
      )
      .toJSON();
    expect(photosGrid).toMatchSnapshot();
  });

  test('onSectionRendered calls onRowsRendered with the correct indices', () => {
    const photosGrid = filteredPhotosGrid.root.instance;
    photosGrid.columnCount = 5;

    photosGrid.onRowsRendered = jest.fn();

    photosGrid.onSectionRendered({
      columnStartIndex: 1,
      columnStopIndex: 3,
      rowStartIndex: 2,
      rowStopIndex: 4
    });

    expect(photosGrid.onRowsRendered.mock.calls.length).toBe(1);
    expect(photosGrid.onRowsRendered.mock.calls[0].length).toBe(1);
    expect(photosGrid.onRowsRendered.mock.calls[0][0]).toEqual({
      startIndex: 11,
      stopIndex: 23
    });
  });

  test('cellRenderer calls rowRenderer with the correct photo index', () => {
    const photosGrid = filteredPhotosGrid.root.instance;
    photosGrid.columnCount = 5;

    const originalRowRenderer = photosGrid.rowRenderer;
    photosGrid.rowRenderer = jest.fn();

    photosGrid.cellRenderer({
      columnIndex: 1,
      key: 5,
      rowIndex: 3,
      style: { height: 'auto' }
    });

    expect(photosGrid.rowRenderer.mock.calls.length).toBe(1);
    expect(photosGrid.rowRenderer.mock.calls[0].length).toBe(1);
    expect(photosGrid.rowRenderer.mock.calls[0][0]).toEqual({
      index: 16,
      key: 5,
      style: { height: 'auto' }
    });

    photosGrid.rowRenderer = originalRowRenderer;
  });

  test('rowRenderer returns a PhotoThumbnail for the photo at the given index', () => {
    const photosGrid = filteredPhotosGrid.root.instance;

    const photoThumbnail = photosGrid.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });
    expect(photoThumbnail).toMatchSnapshot();
  });

  test('rowRender should set the PhotoThumbnail handleSelect to call the handlePhotoSelect callback', () => {
    const photosGrid = filteredPhotosGrid.root.instance;

    const photoThumbnail = photosGrid.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });

    photoThumbnail.props.handleSelect();

    expect(mockHandlePhotoSelect.mock.calls.length).toBe(1);
    expect(mockHandlePhotoSelect.mock.calls[0].length).toBe(1);
    expect(mockHandlePhotoSelect.mock.calls[0][0]).toBe(photos[0]);
  });

  test('rowHeight scales the height of the photo at the given index to match a width of 272', () => {
    const photosGrid = filteredPhotosGrid.root.instance;
    photosGrid.columnCount = 2;

    expect(photosGrid.rowHeight({ index: 0 })).toBe(75);
  });

  test('isRowLoaded returns false if the photo at the given index has loaded = false', () => {
    const photosGrid = filteredPhotosGrid.root.instance;

    expect(photosGrid.isRowLoaded({ index: 0 })).toBe(false);
  });

  test('isRowLoaded returns true if the photo at the given index has loaded = true', () => {
    const photosGrid = filteredPhotosGrid.root.instance;

    expect(photosGrid.isRowLoaded({ index: 1 })).toBe(true);
  });

  test('loadMoreRows should call the getAndStoreLocations callback', () => {
    const photosGrid = filteredPhotosGrid.root.instance;

    photosGrid.loadMoreRows({ startIndex: 0, stopIndex: 1 });
    expect(mockGetAndStoreLocations.mock.calls.length).toBe(1);
    expect(mockGetAndStoreLocations.mock.calls[0]).toEqual([0, 1]);
  });

  test('forceUpdate should force the grid to update and reset the loaded rows cache', () => {
    const photosGrid = filteredPhotosGrid.root.instance;

    photosGrid.grid.recomputeGridSize = jest.fn();
    photosGrid.loader.resetLoadMoreRowsCache = jest.fn();

    photosGrid.forceUpdate();

    expect(photosGrid.grid.recomputeGridSize.mock.calls.length).toBe(1);
    expect(photosGrid.loader.resetLoadMoreRowsCache.mock.calls.length).toBe(1);
    expect(photosGrid.loader.resetLoadMoreRowsCache.mock.calls[0]).toEqual([
      true
    ]);
  });
});
