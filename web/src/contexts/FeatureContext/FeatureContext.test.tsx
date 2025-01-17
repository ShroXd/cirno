import { ReactNode, useContext } from 'react'

import { act, renderHook } from '@testing-library/react'
import { afterEach, beforeEach, describe, expect, it, vi } from 'vitest'

import { FeatureContext, FeatureProvider } from './FeatureContext'
import { defaultFeatures } from '~/config/feature'

describe('FeatureContext', async () => {
  beforeEach(() => {
    vi.stubGlobal('localStorage', {
      getItem: vi.fn(),
      setItem: vi.fn(),
      removeItem: vi.fn(),
      clear: vi.fn(),
    })
  })

  afterEach(() => {
    vi.restoreAllMocks()
  })

  const wrapper = ({ children }: { children: ReactNode }) => (
    <FeatureProvider>{children}</FeatureProvider>
  )

  it('should provide context to child components', () => {
    const { result } = renderHook(() => useContext(FeatureContext), {
      wrapper,
    })

    expect(result.current).toBeDefined()
    expect(result.current?.features).toBeDefined()
  })

  it('should be able to check if a feature is enabled', () => {
    const features = [
      { id: 'feature1', enabled: false },
      { id: 'feature2', enabled: true },
    ]

    vi.spyOn(localStorage, 'getItem').mockReturnValue(JSON.stringify(features))

    const { result } = renderHook(() => useContext(FeatureContext), {
      wrapper,
    })
    const { isFeatureEnabled } = result.current

    expect(isFeatureEnabled('feature1')).toBe(false)
    expect(isFeatureEnabled('feature2')).toBe(true)
    expect(isFeatureEnabled('feature3')).toBe(false)
  })

  it('should be able to toggle a feature', async () => {
    const features = [
      { id: 'feature1', enabled: false },
      { id: 'feature2', enabled: false },
    ]

    vi.spyOn(localStorage, 'getItem').mockReturnValue(JSON.stringify(features))
    const setItem = vi.spyOn(localStorage, 'setItem')

    const { result } = renderHook(() => useContext(FeatureContext), {
      wrapper,
    })

    expect(result.current?.features).toEqual([
      { id: 'feature1', enabled: false },
      { id: 'feature2', enabled: false },
    ])
    expect(setItem).toHaveBeenCalledTimes(1)

    act(() => {
      result.current?.toggleFeature('feature1')
    })

    expect(result.current?.features).toEqual([
      { id: 'feature1', enabled: true },
      { id: 'feature2', enabled: false },
    ])
    expect(setItem).toHaveBeenCalledTimes(2)
    expect(setItem).toHaveBeenCalledWith(
      'features',
      JSON.stringify([
        { id: 'feature1', enabled: true },
        { id: 'feature2', enabled: false },
      ])
    )
  })

  it('should be able to reset features', async () => {
    const features = [
      { id: 'feature1', enabled: false },
      { id: 'feature2', enabled: false },
    ]

    vi.spyOn(localStorage, 'getItem').mockReturnValue(JSON.stringify(features))
    const setItem = vi.spyOn(localStorage, 'setItem')

    const { result } = renderHook(() => useContext(FeatureContext), {
      wrapper,
    })

    expect(result.current?.features).toEqual([
      { id: 'feature1', enabled: false },
      { id: 'feature2', enabled: false },
    ])
    expect(setItem).toHaveBeenCalledTimes(1)

    act(() => {
      result.current?.resetFeature()
    })

    expect(result.current?.features).toEqual(defaultFeatures)
    expect(setItem).toHaveBeenCalledTimes(2)
    expect(setItem).toHaveBeenCalledWith(
      'features',
      JSON.stringify(defaultFeatures)
    )
  })
})
