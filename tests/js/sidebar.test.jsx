import React from 'react';
import renderer from 'react-test-renderer';
import Sidebar from '../../src/gui/js/sidebar';

describe('Sidebar', () => {
  let mockGetAndStoreLocations;
  let mockHandlePhotoSelect;
  let filteredSidebar;

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

    filteredSidebar = renderer.create(
      <Sidebar
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
    const sidebar = renderer
      .create(
        <Sidebar
          photos={photos}
          getAndStoreLocations={mockGetAndStoreLocations}
          handlePhotoSelect={mockHandlePhotoSelect}
        />
      )
      .toJSON();
    expect(sidebar).toMatchSnapshot();
  });

  test('onSectionRendered calls onRowsRendered with the correct indices', () => {
    const sidebar = filteredSidebar.root.instance;
    sidebar.columnCount = 5;

    sidebar.onRowsRendered = jest.fn();

    sidebar.onSectionRendered({
      columnStartIndex: 1,
      columnStopIndex: 3,
      rowStartIndex: 2,
      rowStopIndex: 4
    });

    expect(sidebar.onRowsRendered.mock.calls.length).toBe(1);
    expect(sidebar.onRowsRendered.mock.calls[0].length).toBe(1);
    expect(sidebar.onRowsRendered.mock.calls[0][0]).toEqual({
      startIndex: 11,
      stopIndex: 23
    });
  });

  test('cellRenderer calls rowRenderer with the correct photo index', () => {
    const sidebar = filteredSidebar.root.instance;
    sidebar.columnCount = 5;

    const originalRowRenderer = sidebar.rowRenderer;
    sidebar.rowRenderer = jest.fn();

    sidebar.cellRenderer({
      columnIndex: 1,
      key: 5,
      rowIndex: 3,
      style: { height: 'auto' }
    });

    expect(sidebar.rowRenderer.mock.calls.length).toBe(1);
    expect(sidebar.rowRenderer.mock.calls[0].length).toBe(1);
    expect(sidebar.rowRenderer.mock.calls[0][0]).toEqual({
      index: 16,
      key: 5,
      style: { height: 'auto' }
    });

    sidebar.rowRenderer = originalRowRenderer;
  });

  test('rowRenderer returns a PhotoThumbnail for the photo at the given index', () => {
    const sidebar = filteredSidebar.root.instance;

    const photoThumbnail = sidebar.rowRenderer({
      index: 0,
      key: 1,
      style: { color: 'black' }
    });
    expect(photoThumbnail).toMatchSnapshot();
  });

  test('rowRender should set the PhotoThumbnail handleSelect to call the handlePhotoSelect callback', () => {
    const sidebar = filteredSidebar.root.instance;

    const photoThumbnail = sidebar.rowRenderer({
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
    const sidebar = filteredSidebar.root.instance;
    sidebar.columnCount = 2;

    expect(sidebar.rowHeight({ index: 0 })).toBe(75);
  });

  test('isRowLoaded returns false if the photo at the given index has loaded = false', () => {
    const sidebar = filteredSidebar.root.instance;

    expect(sidebar.isRowLoaded({ index: 0 })).toBe(false);
  });

  test('isRowLoaded returns true if the photo at the given index has loaded = true', () => {
    const sidebar = filteredSidebar.root.instance;

    expect(sidebar.isRowLoaded({ index: 1 })).toBe(true);
  });

  test('loadMoreRows should call the getAndStoreLocations callback', () => {
    const sidebar = filteredSidebar.root.instance;

    sidebar.loadMoreRows({ startIndex: 0, stopIndex: 1 });
    expect(mockGetAndStoreLocations.mock.calls.length).toBe(1);
    expect(mockGetAndStoreLocations.mock.calls[0]).toEqual([0, 1]);
  });

  test('forceUpdate should force the grid to update and reset the loaded rows cache', () => {
    const sidebar = filteredSidebar.root.instance;

    sidebar.grid.recomputeGridSize = jest.fn();
    sidebar.loader.resetLoadMoreRowsCache = jest.fn();

    sidebar.forceUpdate();

    expect(sidebar.grid.recomputeGridSize.mock.calls.length).toBe(1);
    expect(sidebar.loader.resetLoadMoreRowsCache.mock.calls.length).toBe(1);
    expect(sidebar.loader.resetLoadMoreRowsCache.mock.calls[0]).toEqual([true]);
  });
});
