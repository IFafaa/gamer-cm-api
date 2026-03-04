import { cn } from '@/lib/utils';

describe('cn (className utility)', () => {
  it('returns empty string when no args are passed', () => {
    expect(cn()).toBe('');
  });

  it('merges single class string', () => {
    expect(cn('foo')).toBe('foo');
  });

  it('merges multiple class strings', () => {
    expect(cn('foo', 'bar')).toBe('foo bar');
  });

  it('deduplicates conflicting tailwind classes (last wins)', () => {
    // tailwind-merge: p-4 wins over p-2
    expect(cn('p-2', 'p-4')).toBe('p-4');
  });

  it('handles conditional classes (falsy values ignored)', () => {
    expect(cn('base', false && 'ignored', undefined, null, 'active')).toBe(
      'base active'
    );
  });

  it('handles object syntax from clsx', () => {
    expect(cn({ 'text-red-500': true, 'text-blue-500': false })).toBe(
      'text-red-500'
    );
  });

  it('handles array syntax from clsx', () => {
    expect(cn(['a', 'b'], 'c')).toBe('a b c');
  });
});
