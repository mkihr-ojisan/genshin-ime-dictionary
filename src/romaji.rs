pub fn romaji_to_hiragana(romaji: &str, useー: bool) -> String {
    let mut hiragana = String::new();

    let mut chars = romaji.char_indices().peekable();

    while let Some((i, next)) = chars.next() {
        match next.to_ascii_lowercase() {
            'a' => {
                if useー && i > 0 && &romaji[i - 1..=i - 1] == "a" {
                    hiragana.push('ー');
                } else {
                    hiragana.push('あ');
                }
            }
            'i' => {
                if useー && i > 0 && &romaji[i - 1..=i - 1] == "i" {
                    hiragana.push('ー');
                } else {
                    hiragana.push('い');
                }
            }
            'u' => {
                if useー && i > 0 && &romaji[i - 1..=i - 1] == "u" {
                    hiragana.push('ー');
                } else {
                    hiragana.push('う');
                }
            }
            'e' => {
                if useー && i > 0 && &romaji[i - 1..=i - 1] == "e" {
                    hiragana.push('ー');
                } else {
                    hiragana.push('え');
                }
            }
            'o' => {
                if useー && i > 0 && &romaji[i - 1..=i - 1] == "o" {
                    hiragana.push('ー');
                } else {
                    hiragana.push('お');
                }
            }
            'k' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('か');
                }
                'i' => {
                    chars.next();
                    hiragana.push('き');
                }
                'u' => {
                    chars.next();
                    hiragana.push('く');
                }
                'e' => {
                    chars.next();
                    hiragana.push('け');
                }
                'o' => {
                    chars.next();
                    hiragana.push('こ');
                }
                'k' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('き');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('き');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('き');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            's' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('さ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('し');
                }
                'u' => {
                    chars.next();
                    hiragana.push('す');
                }
                'e' => {
                    chars.next();
                    hiragana.push('せ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('そ');
                }
                's' => {
                    hiragana.push('っ');
                }
                'h' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('し');
                            hiragana.push('ゃ');
                        }
                        'i' => {
                            hiragana.push('し');
                        }
                        'u' => {
                            hiragana.push('し');
                            hiragana.push('ゅ');
                        }
                        'e' => {
                            hiragana.push('し');
                            hiragana.push('ぇ');
                        }
                        'o' => {
                            hiragana.push('し');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('し');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('し');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('し');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            't' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('た');
                }
                'i' => {
                    chars.next();
                    hiragana.push('て');
                    hiragana.push('ぃ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('と');
                    hiragana.push('ぅ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('て');
                }
                'o' => {
                    chars.next();
                    hiragana.push('と');
                }
                't' => {
                    hiragana.push('っ');
                }
                's' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('つ');
                            hiragana.push('ぁ');
                        }
                        'i' => {
                            hiragana.push('つ');
                            hiragana.push('ぃ');
                        }
                        'u' => hiragana.push('つ'),
                        'e' => {
                            hiragana.push('つ');
                            hiragana.push('ぇ');
                        }
                        'o' => {
                            hiragana.push('つ');
                            hiragana.push('ぉ');
                        }
                        _ => {}
                    }
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('ち');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('ち');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('ち');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'c' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'h' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('ち');
                            hiragana.push('ゃ');
                        }
                        'i' => {
                            hiragana.push('ち');
                        }
                        'u' => {
                            hiragana.push('ち');
                            hiragana.push('ゅ');
                        }
                        'e' => {
                            hiragana.push('ち');
                            hiragana.push('ぇ');
                        }
                        'o' => {
                            hiragana.push('ち');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                'c' => {
                    hiragana.push('っ');
                }
                _ => {}
            },
            'n' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                hiragana.push('ん');
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('な');
                }
                'i' => {
                    chars.next();
                    hiragana.push('に');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ぬ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('ね');
                }
                'o' => {
                    chars.next();
                    hiragana.push('の');
                }
                'n' => {
                    hiragana.push('ん');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('に');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('に');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('に');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {
                    hiragana.push('ん');
                }
            },
            'h' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('は');
                }
                'i' => {
                    chars.next();
                    hiragana.push('ひ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ふ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('へ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ほ');
                }
                'h' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('ひ');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('ひ');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('ひ');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'f' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ふ');
                    hiragana.push('ぁ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('ふ');
                    hiragana.push('ぃ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ふ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('ふ');
                    hiragana.push('ぇ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ふ');
                    hiragana.push('ぉ');
                }
                'f' => {
                    hiragana.push('っ');
                }
                _ => {}
            },
            'm' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ま');
                }
                'i' => {
                    chars.next();
                    hiragana.push('み');
                }
                'u' => {
                    chars.next();
                    hiragana.push('む');
                }
                'e' => {
                    chars.next();
                    hiragana.push('め');
                }
                'o' => {
                    chars.next();
                    hiragana.push('も');
                }
                'm' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('み');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('み');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('み');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'y' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('や');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ゆ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('よ');
                }
                'y' => {
                    hiragana.push('っ');
                }
                _ => {}
            },
            'r' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ら');
                }
                'i' => {
                    chars.next();
                    hiragana.push('り');
                }
                'u' => {
                    chars.next();
                    hiragana.push('る');
                }
                'e' => {
                    chars.next();
                    hiragana.push('れ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ろ');
                }
                'r' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('り');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('り');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('り');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'w' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('わ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('う');
                    hiragana.push('ぃ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('う');
                    hiragana.push('ぇ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('を');
                }
                'w' => {
                    hiragana.push('っ');
                }
                _ => {}
            },
            'g' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('が');
                }
                'i' => {
                    chars.next();
                    hiragana.push('ぎ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ぐ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('げ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ご');
                }
                'g' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('ぎ');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('ぎ');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('ぎ');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'z' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ざ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('じ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ず');
                }
                'e' => {
                    chars.next();
                    hiragana.push('ぜ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ぞ');
                }
                'z' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('じ');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('じ');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('じ');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'j' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('じ');
                    hiragana.push('ゃ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('じ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('じ');
                    hiragana.push('ゅ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('じ');
                    hiragana.push('ぇ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('じ');
                    hiragana.push('ょ');
                }
                _ => {}
            },
            'd' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('だ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('で');
                    hiragana.push('ぃ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ど');
                    hiragana.push('ぅ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('で');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ど');
                }
                'd' => {
                    hiragana.push('っ');
                }
                _ => {}
            },
            'b' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ば');
                }
                'i' => {
                    chars.next();
                    hiragana.push('び');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ぶ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('べ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ぼ');
                }
                'b' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('び');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('び');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('び');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            'v' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ゔ');
                    hiragana.push('ぁ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('ゔ');
                    hiragana.push('ぃ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ゔ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('ゔ');
                    hiragana.push('ぇ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ゔ');
                    hiragana.push('ぉ');
                }
                'v' => {
                    hiragana.push('っ');
                }
                _ => {}
            },
            'p' => match if let Some((_, peek)) = chars.peek() {
                peek
            } else {
                break;
            }
            .to_ascii_lowercase()
            {
                'a' => {
                    chars.next();
                    hiragana.push('ぱ');
                }
                'i' => {
                    chars.next();
                    hiragana.push('ぴ');
                }
                'u' => {
                    chars.next();
                    hiragana.push('ぷ');
                }
                'e' => {
                    chars.next();
                    hiragana.push('ぺ');
                }
                'o' => {
                    chars.next();
                    hiragana.push('ぽ');
                }
                'p' => {
                    hiragana.push('っ');
                }
                'y' => {
                    chars.next();
                    match if let Some((_, next)) = chars.next() {
                        next
                    } else {
                        break;
                    }
                    .to_ascii_lowercase()
                    {
                        'a' => {
                            hiragana.push('ぴ');
                            hiragana.push('ゃ');
                        }
                        'u' => {
                            hiragana.push('ぴ');
                            hiragana.push('ゅ');
                        }
                        'o' => {
                            hiragana.push('ぴ');
                            hiragana.push('ょ');
                        }
                        _ => {}
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    hiragana
}

#[test]
fn test() {
    assert_eq!(romaji_to_hiragana("kaa", true), "かー");
}
