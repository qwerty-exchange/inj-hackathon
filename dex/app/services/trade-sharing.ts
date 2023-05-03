import { indexerExplorerApi } from '../Services'

export const generateImage = async (position : any = undefined): Promise<any> => {
  const timestamp = (await indexerExplorerApi.fetchBlocks({ limit: 1 })).data[0].timestamp

  const svg = await fetch(position.direction === 'long' ? '/svg/Long.svg' : '/svg/Short.svg')

  const svgDom = new window.DOMParser().parseFromString(await svg.text(), 'text/xml')
  svgDom.getElementById('leverage')!.textContent = position.leverage
  svgDom.getElementById('ticker')!.textContent = position.ticker
  svgDom.getElementById('entryprice')!.textContent = position.entryPrice
  svgDom.getElementById('lastprice')!.textContent = position.lastprice
  svgDom.getElementById('time')!.textContent = timestamp.slice(0, 19) + ' UTC'

  const pnl = position.pnl.toString()
  svgDom.getElementById('pnl')!.textContent = pnl
  if (pnl[0] !== '-') {
    svgDom.getElementById('pnlText')!.setAttribute('fill', 'url(#pnl_green)')
  } else {
    svgDom.getElementById('pnlText')!.setAttribute('fill', 'url(#pnl_red)')
  }

  const url = await svgToPng(new XMLSerializer().serializeToString(svgDom))

  const image = new Image()
  image.src = url as any
  const w = window.open('') as Window
  w.document.write(
    '<iframe src="' +
    url +
      '" frameborder="0" style="border:0; top:0px; left:0px; bottom:0px; right:0px; width:100%; height:100%;" allowfullscreen></iframe>'
  )

}

const svgToPng = function (svgText: any) {
  // convert an svg text to png using the browser
  return new Promise(function (resolve) {
    try {
      // can use the domUrl function from the browser
      const domUrl = window.URL || window.webkitURL || window
      if (!domUrl) {
        throw new Error('(browser doesnt support this)')
      }

      // figure out the height and width from svg text
      let match = svgText.match(/height="(\d+)/m)
      const height = match && match[1] ? parseInt(match[1], 10) : 100

      match = svgText.match(/width="(\d+)/m)
      const width = match && match[1] ? parseInt(match[1], 10) : 100

      // it needs a namespace
      if (!svgText.match(/xmlns="/mi)) {
        svgText = svgText.replace('<svg ', '<svg xmlns="http://www.w3.org/2000/svg" ')
      }

      // create a canvas element to pass through
      const canvas = document.createElement('canvas')
      canvas.width = width
      canvas.height = height
      const ctx = canvas.getContext('2d')

      // make a blob from the svg
      const svg = new Blob([svgText], {
        type: 'image/svg+xml;charset=utf-8'
      })

      // create a dom object for that image
      const url = domUrl.createObjectURL(svg)

      // create a new image to hold it the converted type
      const img = new Image()

      // when the image is loaded we can get it as base64 url
      img.onload = function () {
        // draw it to the canvas
        ctx?.drawImage(img, 0, 0)

        // we don't need the original any more
        domUrl.revokeObjectURL(url)
        // now we can resolve the promise, passing the base64 url
        resolve(canvas.toDataURL('img/png'))
      }
      // load the image
      img.src = url
    } catch (err) {
      throw new Error('failed to convert svg to png ' + err)
    }
  })
}
